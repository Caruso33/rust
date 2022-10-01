// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:
// * Use a variable set to any integer
// * Use a match expression to determine which message to display
// * Use an underscore (_) to match on any value

fn main() {
    let var = 4;

    match var {
        1 => println!("it's 1"),
        2 => println!("it's 2"),
        3 => println!("it's 3"),
        _ => println!("it's some other number")
    }
}
