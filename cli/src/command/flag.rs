use std::cmp::{Eq, PartialEq};
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::hash::{Hash, Hasher};

pub type FlagSpecSet = HashSet<FlagSpec>;
pub type FlagSet<'a> = HashSet<Flag<'a>>;

#[derive(Debug)]
pub struct UnknownFlagError(pub FlagQuery);

impl fmt::Display for UnknownFlagError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unrecognized flag '{}'", self.0)
    }
}

impl Error for UnknownFlagError {}

#[derive(Debug)]
pub struct FlagMissingArgError(pub FlagQuery);

impl fmt::Display for FlagMissingArgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unrecognzied flag '{}'", self.0)
    }
}

impl Error for FlagMissingArgError {}

/// Flag argument specification. Flags can come with no argument, optional
/// argument, or required argument.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum ArgSpec {
    #[default]
    None,
    Optional,
    Required,
}

/// Flag argument. This is duplicated from ArgSpec, because the user is
/// expected to configure flags using ArgSpec and then during runtime
/// at user input, the commands are parsed and argument values are 
/// populated inside Arg.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum Arg {
    #[default]
    None,
    Optional(Option<String>),
    Required(String),
}

impl Arg {
    pub fn raw(&self) -> Option<String> {
        match self {
            Arg::Optional(val) => val.clone(),
            Arg::Required(s) => Some(s.to_owned()),
            _ => None,
        }
    }

    pub fn get<T: From<String>>(&self) -> Result<Option<T>, Box<dyn Error>> {
        let raw = self.raw();
        if let None = raw {
            return Ok(None);
        }
        Ok(Some(T::from(raw.unwrap())))
    }
}

/// Use this to uniquely identify Flag
#[derive(Clone, Debug, Eq)]
pub struct FlagId {
    name: String,
    short: char,
}

impl Hash for FlagId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        format!("{}:{}", self.name, self.short).hash(state);
    }
}

impl PartialEq for FlagId {
    fn eq(&self, other: &Self) -> bool {
        format!("{}:{}", self.name, self.short) == format!("{}:{}", other.name, other.short)
    }
}

/// Use this to search or compare flags based on text data
#[derive(Debug)]
pub enum FlagQuery {
    Name(String),
    Short(char),
}

impl fmt::Display for FlagQuery {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let flag_str = match self {
            FlagQuery::Name(s) => { format!("--{}", s) },
            FlagQuery::Short(c) => { format!("-{}", c) },
        };
        write!(f, "{}", flag_str)
    }
}

pub fn query_flag_spec<'a>(needle: &FlagQuery, haystack: &'a FlagSpecSet) -> Option<&'a FlagSpec> {
    for entry in haystack.iter() {
        if match needle {
            FlagQuery::Name(ref s) => *s == entry.id.name,
            FlagQuery::Short(ref c) => *c == entry.id.short,
        } {
            return Some(&entry);
        }
    }
    return None;
}

pub fn query_flag<'a>(needle: &FlagQuery, haystack: &'a FlagSet) -> Option<&'a Flag<'a>> {
    for entry in haystack.iter() {
        if match needle {
            FlagQuery::Name(ref s) => *s == entry.spec.id.name,
            FlagQuery::Short(ref c) => *c == entry.spec.id.short,
        } {
            return Some(&entry);
        }
    }
    return None;
}

/// Use FlagSpec to configure command line options for Commands
#[derive(Clone, Debug, Eq)]
pub struct FlagSpec {
    id: FlagId,
    help: String,
    arg_spec: ArgSpec,
}

impl FlagSpec {
    pub fn new(name: &str, short: char, arg_spec: ArgSpec, help: &str) -> FlagSpec {
        let id = FlagId { name: name.to_owned(), short: short.to_owned() };
        FlagSpec { id, arg_spec, help: help.to_owned() }
    }

    pub fn get_arg_spec(&self) -> &ArgSpec {
        &self.arg_spec
    }

    pub fn help(&self) -> String {
        self.help.clone()
    }
}

impl Hash for FlagSpec {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for FlagSpec {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

/// A Flag is a specific instance of a flag passed into Command.
#[derive(Clone, Debug, Eq)]
pub struct Flag<'a> {
    spec: &'a FlagSpec,
    arg: Arg,
}

impl<'a> Flag<'a> {
    pub fn new(spec: &FlagSpec, arg: Arg) -> Flag {
        Flag { spec, arg }
    }

    pub fn spec(&self) -> &'a FlagSpec {
        self.spec
    }

    pub fn set_arg(&mut self, arg: Option<String>) -> Result<(), &str> {
        self.arg = match self.arg {
            Arg::Optional(_) => { Arg::Optional(arg) },
            Arg::Required(_) => { Arg::Required(arg.expect("Passed None to Arg::Required")) },
            _ => {
                if let Some(_) = arg {
                    return Err("Trying to set value of Arg::None");
                }
                Arg::None
            }
        };
        Ok(())
    }

    pub fn get_arg(&self) -> &Arg {
        &self.arg
    }
}

impl<'a> Hash for Flag<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.spec.id.hash(state);
    }
}

impl<'a> PartialEq for Flag<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.spec.id == other.spec.id
    }
}

/// check if a string is a long flag
pub fn is_long(flag_text: &str) -> bool {
    flag_text.starts_with("--") && flag_text.len() > 3
}

/// check if a string is a short flag
pub fn is_short(flag_text: &str) -> bool {
    flag_text.starts_with("-") && flag_text.len() > 1
}

/// check if a string is a flag
pub fn is_flag(flag_text: &str) -> bool {
    is_long(&flag_text) || is_short(&flag_text)
}

/// convert text string to flag query; if text is not a flag, return None
pub fn extract_flag(flag_text: &str) -> Option<FlagQuery> {
    if is_long(&flag_text) {
        Some(FlagQuery::Name(flag_text.strip_prefix("--").unwrap().to_string()))
    } else if is_short(&flag_text) {
        // short flags are complicated
        // you can have the follwing forms:
        // 1) -a [optarg] e.g. -a myarg
        // 2) -a[optarg] e.g. -amyarg
        // 3) -abc e.g. -a -b -c
        Some(FlagQuery::Short(flag_text.chars().nth(1).unwrap()))
    } else {
        None
    }
}
