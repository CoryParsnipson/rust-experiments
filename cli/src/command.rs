use config::Config;
use crate::shell::Context;
use flag::FlagSet;
use operand::OperandList;
use std::error::Error;

pub mod config;
pub mod flag;
pub mod operand;

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
