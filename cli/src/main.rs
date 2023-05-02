use cli::command;
use cli::command::config;
use cli::command::flag::{self, FlagSpec, FlagSpecSet};
use cli::shell;

fn main() {
    // create a config
    let mut flag_spec = FlagSpecSet::new();
    flag_spec.insert(
        FlagSpec::new("verbose", 'v', flag::ArgSpec::default(),
            "Print more info"
        )
    );
    flag_spec.insert(
        FlagSpec::new( "modulo", 'm', flag::ArgSpec::Required,
            "Perform modulo on the resulting addition"
        )
    );

    let add_config = config::Config::new("add", flag_spec, "Add two numbers together");
    println!("Add command::Config: {:#?}", add_config);

    let res = shell::parse("add 3 4", add_config.get_flags());
    println!("Command string parse: {:#?}", res);

    let res = shell::parse("add -v 3 4", add_config.get_flags());
    println!("Command string parse: {:#?}", res);

    let res = shell::parse("add -v --modulo", add_config.get_flags());
    println!("Command string parse: {:#?}", res);
}
