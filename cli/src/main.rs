use cli::command::{self, Command};
use cli::command::flag::{self, FlagSpec, FlagSpecSet};
use cli::command::operand::MissingOperandError;
use cli::shell::{CommandSet, Context, Shell};
use std::error::Error;

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
    let add_config = command::Config::new(
        "add",
        flag_spec,
        "Add two numbers together",
        | command: &Command, _shell: &Shell, _context: &mut Context | -> Result<(), Box<dyn Error>> {
            let operands = command.operands();
            let expected_num_operands = 2;

            if operands.len() != expected_num_operands {
                return Err(
                    Box::new(
                        MissingOperandError(
                            operands[..].into(),
                            expected_num_operands
                        )
                    )
                );
            }

            println!(
                "{}",
                operands[0].value_as::<i32>()? + operands[1].value_as::<i32>()?
            );

            Ok(())
        },
    );

    let mut command_set = CommandSet::new();
    command_set.insert(add_config.name().to_owned(), add_config);

    let mut context = Context::new();

    let shell = Shell::new(command_set);
    shell.run(&mut context);
}
