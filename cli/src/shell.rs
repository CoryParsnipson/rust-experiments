use crate::command;
use crate::command::flag::{self, Flag, FlagMissingArgError, FlagSet, FlagSpecSet, UnknownFlagError};
use crate::command::operand::{Operand, OperandList};
use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::io::{self, Write};

/// Datastructure to hold a list of command configs for shell use
pub type CommandSpec = HashMap<String, command::Config>;

/// Storage to pass information between commands.
pub type Context = HashMap<String, String>;

/// Contains state for entirety of cli interface
pub struct Shell {
    commands: CommandSpec,
    context: Context,
}

impl Shell {
    pub fn new(commands: CommandSpec, context: Context) -> Shell {
        Shell { commands, context }
    }

    pub fn find_command_config(&self, command_name: &str) -> Option<&command::Config> {
        self.commands.get(command_name)
    }

    pub fn find_command_config_mut(&mut self, command_name: &str) -> Option<&mut command::Config> {
        self.commands.get_mut(command_name)
    }

    pub fn get_context(&self) -> &Context { &self.context }
    pub fn get_context_mut(&mut self) -> &mut Context { &mut self.context }

    /// run the shell with this function
    pub fn run(&self) {
        // TODO: print help message (and welcome message?)

        'run: loop {
            print!("{} ", self.make_shell_prompt());
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line");

            println!("User input: {}", input); // DELETEME
            let input = input.trim();

            // TODO: parse command
            // TODO: match ParsedInput to command::Config
            // TODO: create Command struct using command::Config and ParsedInput
            // TODO: execute command and update self.context
        }
    }

    fn make_shell_prompt(&self) -> String {
        "#>".into() // TODO: implement me
    }
}


/// Structured data that is obtained when a string containing a command
/// (presumably entered by a user) is successfully parsed.
#[derive(Debug, Default)]
pub struct ParsedInput<'a> {
    pub command: Option<String>,
    pub options: Option<FlagSet<'a>>,
    pub operands: Option<OperandList>,
}

/// Take a string that is presumably a valid cli command and turn it into
/// structured data
pub fn parse<'a>(input_text: &str, flags: &'a FlagSpecSet) -> Result<ParsedInput<'a>, Box<dyn Error>> {
    println!("Command string: {}", input_text); // DELETEME

    let mut parsed: ParsedInput::<'a> = Default::default();
    let mut tokens: VecDeque<&str> = input_text.split_whitespace().collect();

    // the first token should always be the command name itself
    parsed.command = Some(tokens.pop_front().unwrap().to_owned());

    while !tokens.is_empty() {
        let token = tokens.pop_front().unwrap();
        println!("Found token: {}", token); // DELETEME

        if flag::is_flag(&token) {
            let flag_id = flag::extract_flag(&token).unwrap();
            let spec = flag::query_flag_spec(&flag_id, &flags);
            if spec.is_none() {
                return Err(Box::new(UnknownFlagError(flag_id)));
            }
            let spec = spec.unwrap();
                
            println!("Matched flag spec: {:?}", flag_id); // DELETEME

            // check the argument spec and consume next token if necessary
            let next = tokens.pop_front();
            let parsed_arg = match spec.get_arg_spec() {
                flag::ArgSpec::Optional => {
                    if next.is_none() || flag::is_flag(next.unwrap()) {
                        if next.is_some() {
                            tokens.push_front(next.unwrap()); // don't forget to put next token back
                        }
                        continue;
                    }
                    flag::Arg::Optional(Some(next.unwrap().to_string()))
                },
                flag::ArgSpec::Required => {
                    if next.is_none() || flag::is_flag(next.unwrap()) {
                        return Err(Box::new(FlagMissingArgError(flag_id)));
                    }
                    flag::Arg::Required(next.unwrap().to_string())
                },
                _ => {
                    if next.is_some() {
                        tokens.push_front(next.unwrap()); // don't forget to put next token back
                    }
                    flag::Arg::None
                },
            };

            // add new Flag to parsed input
            if parsed.options.is_none() {
                parsed.options = Some(FlagSet::<'a>::new());
            }

            if let Some(ref mut flag_set) = parsed.options {
                // it is not an error to pass in the same flag multiple times
                // a later value should overwrite an earlier one
                flag_set.replace(Flag::<'a>::new(&spec, parsed_arg));
            }
        } else {
            println!("Found operand: {}", token); // DELETEME

            if parsed.operands.is_none() {
                parsed.operands = Some(OperandList::new());
            }

            if let Some(ref mut operand_list) = parsed.operands {
                operand_list.push(Operand::new(token));
            }
        }
    }

    Ok(parsed)
}
