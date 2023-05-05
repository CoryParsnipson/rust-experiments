pub use config::*;

use crate::shell::{Context, Shell};
use flag::FlagSet;
use operand::OperandList;
use std::error::Error;

pub mod flag;
pub mod operand;

mod config;

#[derive(Debug)]
pub struct Command<'a> {
    config: &'a Config,
    flags: FlagSet<'a>,
    operands: OperandList,
}

impl<'a> Command<'a> {
    pub fn new(config: &'a Config, flags: FlagSet<'a>, operands: OperandList) -> Command<'a> {
        Command { config, flags, operands, }
    }

    pub fn execute(&self, shell: &Shell, context: &mut Context) -> Result<(), Box<dyn Error>> {
        (self.config.callback())(&self, shell, context)
    }
    
    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn flags(&self) -> &FlagSet<'a> {
        &self.flags
    }

    pub fn flags_mut(&mut self) -> &mut FlagSet<'a> {
        &mut self.flags
    }

    pub fn operands(&self) -> &OperandList {
        &self.operands
    }

    pub fn operands_mut(&mut self) -> &mut OperandList {
        &mut self.operands
    }
}
