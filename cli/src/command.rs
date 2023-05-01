use std::collections::{HashMap, VecDeque};
use std::error::Error;
use flag::{Flag, FlagMissingArgError, FlagSet, FlagSpecSet, UnknownFlagError};
use operand::{Operand, OperandList};

pub mod flag;
pub mod operand;

/// All specifications to run a Command. Each flag must be unique, according to
/// PartialEq defined on flag::FlagId.
#[derive(Debug)]
pub struct Config {
    name: String,
    flags: FlagSpecSet,
    help: String,
}

impl Config {
    pub fn new(name: &str, flags: FlagSpecSet, help: &str) -> Config {
        Config { name: name.into(), flags, help: help.into() }
    }

    pub fn get_flags(&self) -> &FlagSpecSet {
        &self.flags
    }
}

/// Storage to pass information between commands.
pub type Context = HashMap<String, String>;

/// This is an intermediate structure used to store information before
/// it becomes a command???
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
                                                  //
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

pub struct Command<'a> {
    config: &'a Config,
    options: FlagSet<'a>,
    operands: OperandList,
}

impl<'a> Command<'a> {
    pub fn new(config: &'a Config, options: FlagSet<'a>, operands: OperandList) -> Command<'a> {
        Command { config, options, operands, }
    }

    pub fn execute(&self, context: &mut Context) -> Result<(), Box<dyn Error>> {
        println!("context: {:#?}", context);
        Ok(())
    }
}

//use std::collections::HashMap;
//use std::collections::VecDeque;
//use std::clone::Clone;
//use std::cmp::{Eq, PartialEq};
//use std::hash::{Hash, Hasher};

//pub type CommandArgs = HashMap<Flag, String>;
//pub type CommandContext = HashMap<String, String>;
//pub type CommandCallback<T, E> = fn(&CommandArgs, &CommandContext) -> Result<T, E>;
//
//pub struct Command<T, E> {
//    flags: Vec<Flag>,
//    callback: CommandCallback<T, E>,
//}
//
//impl<T, E> Command<T, E> {
//    pub fn new(flags: Vec<Flag>, callback: CommandCallback<T, E>) -> Command<T, E> {
//        Command::<T, E> {
//            flags,
//            callback,
//        }
//    }
//
//    pub fn execute(&self, args: &CommandArgs, context: &CommandContext) -> Result<T, E> {
//        (self.callback)(args, context)
//    }
//
//    pub fn parse_flags(&self, command_string: &str) -> CommandArgs {
//        println!("Command string: '{}'", command_string); // DELETEME
//        let mut args = HashMap::<Flag, String>::new();
//
//        // skip the first element of the command string because it should be the command identifier
//        let mut tokens: VecDeque<&str> = command_string.split_whitespace().skip(1).collect();
//        while !tokens.is_empty() {
//            let token = tokens.pop_front().unwrap();
//            println!("Found token: {}", token); // DELETEME
//
//            if is_flag(&token) {
//                let flag_id = extract_flag_name(token).unwrap();
//                let flag = self.get_flag(&flag_id).unwrap();
//
//                println!("Flag: {:?}", flag); // DELETEME
//
//                match flag.arg {
//                    FlagArg::Require => {
//                        // TODO: change return type to Result, return error where necessary
//                        let next = tokens.front();
//                        if next == None || is_flag(next.unwrap()) {
//                            panic!("Flag {:?} missing required argument. Received {} instead.", flag, next.unwrap());
//                        }
//                        args.insert(flag.clone(), tokens.pop_front().unwrap().to_owned());
//                    },
//                    FlagArg::Optional => {
//                        let next = tokens.front();
//                        if let Some(next_token) = next {
//                            if !is_flag(next_token) {
//                                args.insert(flag.clone(), tokens.pop_front().unwrap().to_owned());
//                            }
//                        }
//                    }
//                    _ => ()
//                }
//            } else {
//                println!("Operand: {:?}", token); // DELETEME
//            }
//        }
//
//        args
//    }
//
//    /// get a flag from command struct's data based on identifier. The flag can
//    /// be either the long name or short name of the flag.
//    fn get_flag(&self, identifier: &FlagIdx) -> Option<&Flag> {
//        for f in self.flags.iter() {
//            let is_match = match identifier {
//                FlagIdx::Name(ref name) => f.name == *name,
//                FlagIdx::ShortName(ref short) => f.short == *short,
//            };
//
//            if is_match {
//                return Some(&f);
//            }
//        }
//
//        None
//    }
//}
