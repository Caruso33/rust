use super::Command;
use std::io;

pub fn read_user_input(user_comment: Option<&str>) -> io::Result<String> {
    let mut buffer = String::new();

    match user_comment {
        None => println!(),
        Some(text) => println!("{}", text),
    }

    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_owned())
}

pub fn evaluate_user_input(input: &str) -> Option<Command> {
    match input.trim().to_lowercase().as_str() {
        "view" => Some(Command::View),
        "add" => Some(Command::Add),
        "remove" => Some(Command::Remove),
        "edit" => Some(Command::Edit),
        "exit" => Some(Command::Exit),
        _ => None,
    }
}
