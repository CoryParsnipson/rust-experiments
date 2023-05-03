use cli::command;
use cli::command::flag::{self, FlagSpec, FlagSpecSet};
use cli::shell::{CommandSet, Context, Shell};

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

    let mut command_set = CommandSet::new();
    command_set.insert(add_config.name().to_owned(), add_config);

    let shell = Shell::new(command_set, Context::new());
    shell.run();
}
