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
Have fun!\n"
        );
        command = read_user_input(None, None);

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

fn read_user_input(
    user_comment: Option<&str>,
    pre_used_buffer: Option<String>,
) -> io::Result<String> {
    let mut buffer = match pre_used_buffer {
        None => String::new(),
        Some(x) => x,
    };

    match user_comment {
        None => println!("Enter command:"),
        Some(text) => println!("{}", text),
    }

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
        Command::Add => add(bills),
        Command::Remove => println!("Not implemented yet"),
        Command::Edit => println!("Not implemented yet"),
    }
}

fn view(bills: &mut Vec<Bill>) {
    println!("\nCurrent Bills:\n");
    for bill in bills {
        println!("{:?}", bill);
    }
    read_user_input(Some(""), None);
}

fn add(bills: &mut Vec<Bill>) {
    let mut name: io::Result<String>;
    let mut amount: io::Result<String>;
    let mut confirmation: io::Result<String>;

    loop {
        let pre_used_name = None;
        // let pre_used_name = if name.is_ok() && name.unwrap().len() > 0 {
        //     Some(name.unwrap());
        // } else {
        //     None;
        // };

        name = read_user_input(Some("Enter Bill Name"), pre_used_name);

        match name {
            Err(_) => continue,
            Ok(_) => break,
        }
    }

    loop {
        let pre_used_amount = None;
        // let pre_used_amount = if amount.is_ok() && amount.unwrap().len() > 0 {
        //     Some(amount.unwrap());
        // } else {
        //     None;
        // };
        amount = read_user_input(Some("Enter Bill Amount"), pre_used_amount);

        if amount.is_err() {
            continue;
        }

        match &amount {
            Err(_) => continue,
            Ok(a) => {
                let float = a.parse::<f32>();
                match float {
                    Ok(_) => break,
                    Err(_) => continue,
                }
            }
        }
    }

    loop {
        confirmation = read_user_input(
            Some(
                format!(
                    "Name: {}, Amount: {}, is this correct? ([y]es/[n]o/[a]bort)",
                    name.as_ref().unwrap(),
                    amount.as_ref().unwrap()
                )
                .as_str(),
            ),
            None,
        );

        match confirmation {
            Err(_) => continue,
            Ok(c) => {
                if c.to_lowercase() == "y" {
                    let bill = Bill::new(name.unwrap(), amount.unwrap().parse::<f32>().unwrap());
                    bills.push(bill);
                    read_user_input(Some("Bill added..."), None);
                    return;
                } else if c.to_lowercase() == "n" {
                    add(bills);
                    return;
                } else if c.to_lowercase() == "a" {
                    return;
                } else {
                    continue;
                }
            }
        }
    }
}
