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

use std::io;

enum Command {
    View,
    Add,
    Remove,
    Edit,
}

#[derive(Debug)]
struct Bill {
    name: String,
    amount: f32,
}

impl Bill {
    fn new(name: String, amount: f32) -> Self {
        Bill { name, amount }
    }
}

fn main() {
    let bill = Bill::new("first bill".to_string(), 123.32);

    let mut bills: Vec<Bill> = Vec::new();
    bills.push(bill);

    let mut command: io::Result<String>;
    loop {
        command = read_user_input();

        let action: Option<Command> = match command {
            Err(_) => continue,
            Ok(c) => evaluate_user_input(c.as_str()),
        };

        // if let Err(_) = command {
        //     continue;
        // }

        // let action = evaluate_user_input(&command.unwrap());

        match action {
            None => continue,
            Some(x) => perform_action(x, &mut bills),
        }
        // if action.is_none() {
        //     continue;
        // }

        // perform_action(action.unwrap(), &mut bills);
    }
}

fn read_user_input() -> io::Result<String> {
    let mut buffer = String::new();

    println!("Enter command:");

    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_owned())
}

fn evaluate_user_input(input: &str) -> Option<Command> {
    match input.trim().to_lowercase().as_str() {
        "view" => Some(Command::View),
        "add" => Some(Command::Add),
        "remove" => Some(Command::Remove),
        "edit" => Some(Command::Edit),
        _ => None,
    }
}

fn perform_action(command: Command, bills: &mut Vec<Bill>) {
    match command {
        Command::View => view(bills),
        Command::Add => println!("Not implemented yet"),
        Command::Remove => println!("Not implemented yet"),
        Command::Edit => println!("Not implemented yet"),
    }
}

fn view(bills: &mut Vec<Bill>) {
    for bill in bills {
        println!("{:?}", bill);
    }
}