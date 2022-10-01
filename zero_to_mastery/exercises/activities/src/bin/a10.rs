// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn main() {
    let int = 250;

    let var = if int > 100 { true } else { false };

    match var {
        _ => print_result(var),
    }
}

fn print_result(b: bool) {
    if b {
        println!("it's big")
    } else {
        println!("it's small")
    };
}
