use super::{add, edit, input, remove, view};
use std::collections::HashMap;
use std::{io, process};

#[derive(Debug)]
pub struct Bill {
    pub name: String,
    pub amount: f32,
}

impl Bill {
    pub fn new(name: String, amount: f32) -> Self {
        Bill { name, amount }
    }
}

pub enum Command {
    View,
    Add,
    Remove,
    Edit,
    Exit,
}

pub struct MainSystem {}

impl MainSystem {
    pub fn run_main_loop(bills: &mut HashMap<String, Bill>) {
        let mut command: io::Result<String>;

        loop {
            print!("\x1B[2J\x1B[1;1H");
            println!(
                "Welcome to the Ultimate Bill Storage System.\n
    You have following Options:\n
        view -> View current Bills\n
        add -> Add a new Bill to the system\n
        edit -> Edit an existing Bill in the system\n
        remove -> Delete a Bill from the system\n
        exit -> Terminate Ultimate Bill Storage System\n
    Have fun!\n"
            );
            command = input::read_user_input(Some("Enter command:"));

            let action: Option<Command> = match command {
                Err(_) => continue,
                Ok(c) => input::evaluate_user_input(c.as_str()),
            };

            match action {
                None => continue,
                Some(x) => Self::perform_action(x, bills),
            }
        }
    }

    pub fn perform_action(command: Command, bills: &mut HashMap<String, Bill>) {
        match command {
            Command::View => view(bills),
            Command::Add => add(bills),
            Command::Remove => remove(bills),
            Command::Edit => edit(bills),
            Command::Exit => process::exit(0),
        }
    }
}
