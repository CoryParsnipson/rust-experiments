use cli::command;
use cli::command::flag::{self, FlagSpec, FlagSpecSet};

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

    let add_config = command::Config::new("add", flag_spec, "Add two numbers together");
    println!("Add command::Config: {:#?}", add_config);

    let res = command::parse("add -v --modulo 3 4", add_config.get_flags());
    println!("Command string parse: {:#?}", res);
}
