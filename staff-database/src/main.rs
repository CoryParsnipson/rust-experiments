use std::collections::HashMap;
use std::io::{self, Write};

type Departments = HashMap<String, Vec<String>>;

enum Command {
    Add(String, String),
    Exit,
    Help,
    List(String),
    Noop,
}

fn parse_command(input: &str) -> Command {
    let tokens: Vec<String>  = input.split_whitespace().map(str::to_string).collect();
    if tokens.len() < 1 {
        return Command::Noop;
    }

    match tokens[0].to_lowercase().as_str() {
        "exit" => {
            return Command::Exit;
        }
        "add" => {
            let mut to = 0;
            for (idx, w) in tokens.iter().enumerate() {
                if w == "to" {
                    to = idx;
                }
            }

            if to == 0 {
                println!("Received malformed Add command.");
                return Command::Help;
            }

            let name = tokens[1..to].join(" ");
            let department = tokens[to+1..].join(" ");

            if department.is_empty() {
                println!("Invalid department name provided.");
                return Command::Help;
            }

            return Command::Add(name, department);
        }
        "list" => {
            if tokens.len() == 2 && tokens[1].to_lowercase() == "departments" {
                return Command::List("*".to_owned());
            }

            if tokens.len() == 3 && tokens[1].to_lowercase() == "department" {
                return Command::List(tokens[2].to_owned());
            }

            return Command::Help;
        }
        _ => {
            return Command::Help;
        }
    }
}

fn execute(command: &Command, departments: &mut Departments) {
    match command {
        Command::Add(employee, department) => {
            let dep = departments.entry(department.to_owned()).or_insert(Vec::new());
            dep.push(employee.to_owned());
        }
        Command::List(department) => {
            if department == "*" {
                let deps: Vec<&String> = departments.keys().collect();
                println!("Listing departments:\n===================");
                for d in deps {
                    println!("{}", d);
                }
                return;
            }

            if let Some(employees) = departments.get(department) {
                println!("Listing employees for department '{}':\n========================================", department);
                for employee in employees {
                    println!("{}", employee);
                }
            } else {
                println!("No department by the name of '{department}' exists.");
            }
        }
        _ => {
            print_help();
        }
    }
}

fn print_help() {
    println!("Company directory lets you add and retrieve employees to/from departments.\n");
    println!("Available commands:\n");
    println!("==================\n");
    println!("Add: Adds an employee to a specified department.\n    Usage - Add <employee name> to <department name>\n");
    println!("List: List all departments or employees of a specific department.\n    Usage - List departments\n    Usage - List department <department name>\n");
    println!("Help: Print this menu.\n    Usage: Help\n");
    println!("Exit: Exits the program.\n    Usage: Exit\n");
    println!("==================\n");

    println!("Note: all commands are case insensitive. Department names *are* case sensitive.");
}

fn main() {
    let mut departments: Departments = HashMap::new();

    println!("Welcome to company directory. Please enter in a command.");

    loop {
        print!("#> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        let command = parse_command(&input);
        match command {
            Command::Exit => {
                println!("Exiting...");
                break;
            }
            Command::Noop => {
                return;
            }
            _ => {
            }
        }
        execute(&parse_command(&input), &mut departments);
    }
}
