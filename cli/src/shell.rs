use crate::command::{self, Command};
use crate::command::flag::{self, Flag, FlagMissingArgError, FlagSet, UnknownFlagError};
use crate::command::operand::{Operand, OperandList};
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Write};

/// Datastructure to hold a list of command configs for shell use
pub type CommandSet = HashMap<String, command::Config>;

/// Storage to pass information between commands.
pub type Context = HashMap<String, String>;

/// Contains state for entirety of cli interface
pub struct Shell {
    commands: CommandSet,
    context: Context,
}

impl Shell {
    pub fn new(commands: CommandSet, context: Context) -> Shell {
        Shell { commands, context }
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

    pub fn get_context(&self) -> &Context { &self.context }
    pub fn get_context_mut(&mut self) -> &mut Context { &mut self.context }

    /// run the shell with this function
    pub fn run(&self) {
        // TODO: print help message (and welcome message?)

        loop {
            print!("{} ", self.make_shell_prompt());
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line");

            println!("User input: {}", input); // DELETEME
            let input = input.trim();

            let command_name = self.extract_command_name(input);
            if command_name.is_none() {
                // what seems to have happened here is that the user hit "enter"
                // and didn't type in anything, so we received an empty string.
                // This is not a bug, just ignore and redisplay the prompt.
                continue
            }
            let command_name = command_name.unwrap();
            println!("Command name: {:#?}", command_name); // DELETEME

            let command_config = self.find_command_config(command_name);
            if command_config.is_none() {
                println!("Unknown command '{}'", command_name);
                continue
            }

            let command = parse(input, command_config.unwrap());
            if let Ok(c) = command {
                println!("Parsed command: {:#?}", c); // DELETEME
            }

            // TODO: execute command and update self.context
        }
    }

    fn make_shell_prompt(&self) -> String {
        "#>".into() // TODO: implement me
    }

    /// Given a user entered command string, extract the command name (which is
    /// going to be the first argument separated by whitespace).
    fn extract_command_name<'a>(&self, input_text: &'a str) -> Option<&'a str> {
        input_text.split_whitespace().next()
    }
}

/// Take a string that is presumably a valid cli command and turn it into
/// structured data
pub fn parse<'a>(input_text: &str, config: &'a command::Config) -> Result<Command<'a>, Box<dyn Error>> {
    let mut tokens = input_text.split_whitespace().skip(1).peekable();
    let mut command = Command::new(config, FlagSet::new(), OperandList::new());

    while tokens.peek().is_some() {
        let token = tokens.next().unwrap();
        println!("Found token: {}", token); // DELETEME

        if flag::is_flag(&token) {
            let flag_id = flag::extract_flag(&token).unwrap();
            let spec = flag::query_flag_spec(&flag_id, config.get_flags());
            if spec.is_none() {
                return Err(Box::new(UnknownFlagError(flag_id)));
            }
            let spec = spec.unwrap();
                
            println!("Matched flag spec: {:?}", flag_id); // DELETEME

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
            println!("Found operand: {}", token); // DELETEME
            command.operands_mut().push(Operand::new(token));
        }
    }

    Ok(command)
}
