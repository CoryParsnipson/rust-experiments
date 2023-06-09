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
        | command: &Command, _shell: &Shell, _context: &mut Context | -> Result<command::ReturnCode, Box<dyn Error>> {
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

            Ok(command::ReturnCode::Ok)
        },
    );

    let help_config = command::Config::new(
        "help",
        FlagSpecSet::new(),
        "Print this help message",
        | _command: &Command, shell: &Shell, _context: &mut Context | -> Result<command::ReturnCode, Box<dyn Error>> {
            println!("{}", shell.help());
            Ok(command::ReturnCode::Ok)
        },
    );

    let exit_config = command::Config::new(
        "exit",
        FlagSpecSet::new(),
        "Quit the command line interface.",
        | _command: &Command, _shell: &Shell, _context: &mut Context | -> Result<command::ReturnCode, Box<dyn Error>> {
            Ok(command::ReturnCode::Abort)
        },
    );

    let mut command_set = CommandSet::new();
    command_set.insert(add_config.name().to_owned(), add_config);
    command_set.insert(help_config.name().to_owned(), help_config);
    command_set.insert(exit_config.name().to_owned(), exit_config);

    let mut context = Context::new();

    let shell = Shell::new(command_set, "Rudimentary general purpose command line interface.");
    shell.run(&mut context);
}
