use crate::shell;
use std::error::Error;
use super::Command;
use super::flag::FlagSpecSet;

pub type Callback = fn(&Command, &mut shell::Context) -> Result<(), Box<dyn Error>>;

/// All specifications to run a Command. Each flag must be unique, according to
/// PartialEq defined on flag::FlagId.
pub struct Config {
    name: String,
    flags: FlagSpecSet,
    help: String,
    callback: Callback,
}

impl Config {
    pub fn new(name: &str, flags: FlagSpecSet, help: &str, callback: Callback) -> Config {
        Config { name: name.into(), flags, help: help.into(), callback }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn get_flags(&self) -> &FlagSpecSet {
        &self.flags
    }

    pub fn callback(&self) -> &Callback {
        &self.callback
    }
}

impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name.fmt(f)?;
        self.flags.fmt(f)?;
        self.help.fmt(f)?;
        Ok(())
    }
}
