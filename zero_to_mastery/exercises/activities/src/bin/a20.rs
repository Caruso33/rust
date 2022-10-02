// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

use std::io;

enum State {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

fn main() {
    let mut command: String;

    loop {
        command = read_user_input();

        let state = evaluate_user_input(&command);

        if state.is_some() {
            print_answer(state.unwrap());
        }
    }
}

fn read_user_input() -> String {
    let mut input = String::new();

    println!("Enter command:");

    io::stdin().read_line(&mut input).unwrap();

    input
}

fn evaluate_user_input(input: &String) -> Option<State> {
    match input.trim().to_lowercase().as_str() {
        "off" => Some(State::Off),
        "sleep" => Some(State::Sleep),
        "reboot" => Some(State::Reboot),
        "shutdown" => Some(State::Shutdown),
        "hibernate" => Some(State::Hibernate),
        _ => None,
    }
}

fn print_answer(state: State) {
    match state {
        State::Off => println!("Switching off now..."),
        State::Sleep => println!("Sleeping now..."),
        State::Reboot => println!("Rebooting now..."),
        State::Shutdown => println!("Shutting down now..."),
        State::Hibernate => println!("Hibernating now..."),
    }
}
