use std::collections::HashMap;
use std::io;

struct AddCommand {
    employee: String,
    department: String,
}

enum Command {
    Ok(AddCommand),
    Print,
    Quit,
    Error,
}

struct Organization {
    departments: HashMap<String, Vec<String>>,
}

impl Organization {
    fn add_user(&mut self, cmd: AddCommand) {
        if self.departments.contains_key(&cmd.department) {
            match self.departments.get_mut(&cmd.department) {
                Some(list) => list.push(cmd.employee),
                None => panic!("Key exists, but does not set up with proper value"),
            }
        }
        else {
            self.departments.insert(cmd.department, vec![cmd.employee]);
        }
    }

    fn print(&self) {
        for (department, employee_list) in &self.departments {
            println!("Department {}: [{}]", department, employee_list.join(", "));
        }
    }
}

fn main() {
    process();
}

fn process() {
    let mut organization = Organization { departments: HashMap::new() };

    loop {
        let mut user_input = String::new();
        let result = io::stdin().read_line(&mut user_input);
        let user_input = user_input.trim().to_string();

        if let Err(e) = result {
            println!("{}", e.to_string());
            continue;
        }

        let command: Vec<String> = user_input.split(" ").map(|s| s.to_string()).collect();
        let result = check_command(&command);

        match result {
            Command::Ok(cmd) => organization.add_user(cmd),
            Command::Print => organization.print(),
            Command::Quit => break,
            Command::Error => continue,
        }
    }
}

fn check_command(cmd: &Vec<String>) -> Command {
    const TO_INDEX: usize = 2;
    const NAME_INDEX: usize = 1;
    const DEPARTMENT_INDEX: usize = 3;

    let command = cmd.first().map_or(Command::Error, |s| {
        match s.as_ref() {
            "Add" => {
                if cmd.len() == 4 &&
                    cmd[TO_INDEX].eq(&"to".to_string()) {
                    println!("Add employee!");

                    return Command::Ok(AddCommand {
                        employee: cmd[NAME_INDEX].clone(),
                        department: cmd[DEPARTMENT_INDEX].clone(),
                    });
                }

                return Command::Error;
            }
            "Quit" => Command::Quit,
            "Print" => Command::Print,
            _ => Command::Error,
        }
    });

    command
}
