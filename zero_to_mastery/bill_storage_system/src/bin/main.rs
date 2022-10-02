// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.

use library::{add, read_user_input, Bill, Command};
use std::{io, process};

fn main() {
    let mut bills: Vec<Bill> = Vec::new();

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
        command = read_user_input(Some("Enter command:"));

        let action: Option<Command> = match command {
            Err(_) => continue,
            Ok(c) => evaluate_user_input(c.as_str()),
        };

        match action {
            None => continue,
            Some(x) => perform_action(x, &mut bills),
        }
    }
}

fn evaluate_user_input(input: &str) -> Option<Command> {
    match input.trim().to_lowercase().as_str() {
        "view" => Some(Command::View),
        "add" => Some(Command::Add),
        "remove" => Some(Command::Remove),
        "edit" => Some(Command::Edit),
        "exit" => Some(Command::Exit),
        _ => None,
    }
}

fn perform_action(command: Command, bills: &mut Vec<Bill>) {
    match command {
        Command::View => view(bills),
        Command::Add => add(bills),
        Command::Remove => println!("Not implemented yet"),
        Command::Edit => println!("Not implemented yet"),
        Command::Exit => process::exit(0),
    }
}

fn view(bills: &mut Vec<Bill>) {
    println!("\nCurrent Bills:\n");
    for bill in bills {
        println!("{:?}", bill);
    }
    _ = read_user_input(None);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
