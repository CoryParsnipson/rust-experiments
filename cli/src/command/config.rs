use super::flag::{FlagSpecSet};

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
