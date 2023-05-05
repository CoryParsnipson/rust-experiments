use crate::command::{self, Command};
use crate::command::flag::{self, Flag, FlagMissingArgError, FlagSet, UnknownFlagError};
use crate::command::operand::{Operand, OperandList};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Write as fmt_Write};
use std::io::{self, Write};

/// default prompt string
const DEFAULT_PROMPT: &str = "#";
const CONTEXT_PROMPT_STRING: &str = "prompt";
const CONTEXT_ON_RUN_COMMAND: &str = "on_run";

/// Datastructure to hold a list of command configs for shell use
pub type CommandSet = HashMap<String, command::Config>;

/// Storage to pass information between commands.
pub type Context = HashMap<String, String>;

/// Contains state for entirety of cli interface
pub struct Shell {
    commands: CommandSet,
    help: String,
}

impl Shell {
    pub fn new(commands: CommandSet, help: &str) -> Shell {
        Shell { commands, help: help.into() }
    }

    /// Given a command name, query the shell config to see if there is a
    /// matching config. If there is, return a reference to it.
    pub fn find_command_config(&self, command_name: &str) -> Option<&command::Config> {
        self.commands.get(command_name)
    }

    /// Given a command name, query the shell config to see if there is a
    /// matching config. If there is, return a mutable reference to it.
    pub fn find_command_config_mut(&mut self, command_name: &str) -> Option<&mut command::Config> {
        self.commands.get_mut(command_name)
    }

    /// print help function for this Shell
    pub fn help(&self) -> String {
        let mut help_str = format!("{}\n\n", self.help);

        let mut name_width = 0;
        let mut help_width = 0;

        let _tmp: Vec<()>  = self.commands.iter().map(|e| {
            name_width = std::cmp::max(name_width, e.1.name().len() + 1);
            help_width = std::cmp::max(help_width, e.1.help().len() + 1);
        }).collect();

        // do this to avoid having to pull in a formatting crate
        for (_, c) in self.commands.iter() {
            for idx in 0..name_width {
                if idx < name_width - c.name().len() {
                    write!(help_str, "{}", " ").unwrap();
                } else {
                    break;
                }
            }
            write!(help_str, "{}    {}", c.name(), c.help()).unwrap();

            for _ in 0..(help_width - c.help().len()) {
                write!(help_str, "{}", " ").unwrap();
            }
            write!(help_str, "\n").unwrap();
        }

        help_str
    }

    pub fn quit(&self) {
        // any "on_quit" actions should be run here
        println!("Goodbye.\n");
    }

    /// run the shell
    pub fn run(&self, context: &mut Context) {
        let on_run_command = context.get(CONTEXT_ON_RUN_COMMAND)
            .unwrap_or(&String::from("")).clone();

        match self.run_parsed_result(&on_run_command, context) {
            Ok(code) => {
                if let command::ReturnCode::Abort = code {
                    return;
                }
            },
            Err(error) => { println!("{}", error); }
        }

        'run: loop {
            print!("{} ", self.make_shell_prompt(&(*context)));
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line");
            let input = input.trim();

            match self.run_parsed_result(input, context) {
                Ok(code) => {
                    if let command::ReturnCode::Abort = code {
                        self.quit();
                        break 'run;
                    }
                },
                Err(error) => { println!("{}", error); }
            }
        }
    }

    /// generate prompt string
    fn make_shell_prompt(&self, context: &Context) -> String {
        let mut prompt_string = String::from(DEFAULT_PROMPT);
        if let Some(s) = context.get(CONTEXT_PROMPT_STRING) {
            prompt_string = s.clone();
        }

        format!("{}>", prompt_string).into()
    }

    /// Given a user entered command string, extract the command name (which is
    /// going to be the first argument separated by whitespace).
    fn extract_command_name<'a>(&self, input_text: &'a str) -> Option<&'a str> {
        input_text.split_whitespace().next()
    }

    /// go from user input string to Command
    fn parse_user_input<'a>(&'a self, input_text: &'a str) -> Result<Option<Command<'a>>, Box<dyn Error>> {
        let command_name = self.extract_command_name(input_text);
        if command_name.is_none() {
            // what seems to have happened here is that the user hit "enter"
            // and didn't type in anything, so we received an empty string.
            // This is not a bug, just ignore and redisplay the prompt.
            return Ok(None)
        }
        let command_name = command_name.unwrap();

        let command_config = self.find_command_config(command_name);
        if command_config.is_none() {
            return Err(Box::new(UnknownCommandError(command_name.into())));
        }

        parse(input_text, command_config.unwrap())
    }

    /// parse a user input string and run the resulting command or show error.
    /// This does parse_user_input() and then command.execute().
    fn run_parsed_result<'a>(&self, input_text: &str, context: &mut Context) -> Result<command::ReturnCode, Box<dyn Error>> {
        match self.parse_user_input(input_text) {
            Ok(c_opt) => match c_opt {
                Some(command) => command.execute(&self, context),
                None => Ok(command::ReturnCode::Ok),
            },
            Err(error) => Err(error),
        }
    }
}

/// Take a string that is presumably a valid cli command and turn it into
/// structured data
pub fn parse<'a>(input_text: &str, config: &'a command::Config) -> Result<Option<Command<'a>>, Box<dyn Error>> {
    if input_text.is_empty() {
        return Ok(None);
    }

    let mut tokens = input_text.split_whitespace().skip(1).peekable();
    let mut command = Command::new(config, FlagSet::new(), OperandList::new());

    while tokens.peek().is_some() {
        let token = tokens.next().unwrap();

        if flag::is_flag(&token) {
            let flag_id = flag::extract_flag(&token).unwrap();
            let spec = flag::query_flag_spec(&flag_id, config.get_flags());
            if spec.is_none() {
                return Err(Box::new(UnknownFlagError(flag_id)));
            }
            let spec = spec.unwrap();
                
            // check the argument spec and consume next token if necessary
            let next = tokens.peek();
            let parsed_arg = match spec.get_arg_spec() {
                flag::ArgSpec::Optional => {
                    if next.is_none() || flag::is_flag(next.unwrap()) {
                        continue;
                    }
                    flag::Arg::Optional(Some(tokens.next().unwrap().to_string()))
                },
                flag::ArgSpec::Required => {
                    if next.is_none() || flag::is_flag(next.unwrap()) {
                        return Err(Box::new(FlagMissingArgError(flag_id)));
                    }
                    flag::Arg::Required(tokens.next().unwrap().to_string())
                },
                _ => {
                    flag::Arg::None
                },
            };

            // it is not an error to pass in the same flag multiple times a
            // later value should overwrite an earlier one
            command.flags_mut().replace(Flag::<'a>::new(&spec, parsed_arg));
        } else {
            command.operands_mut().push(Operand::new(token));
        }
    }

    Ok(Some(command))
}

#[derive(Debug)]
pub struct UnknownCommandError(pub String);

impl std::fmt::Display for UnknownCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error: unknown command {}", self.0)
    }
}

impl std::error::Error for UnknownCommandError {}
