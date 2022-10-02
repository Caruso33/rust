// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    Some(a / b)
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{} {}", first, second)
}

fn main() {}

#[test]
fn it_returns_correct_bound_of_clamp() {
    assert_eq!(clamp(5, 2, 4), 4);
    assert_eq!(clamp(2, 3, 4), 3);
    assert_eq!(clamp(3, 2, 4), 3);
}

#[test]
fn it_returns_division() {
    assert_eq!(div(4, 2).unwrap(), 2);
    assert_eq!(div(5, 2).unwrap(), 2);
    assert_eq!(div(6, 2).unwrap(), 3);
}

#[test]
fn it_correctly_concatenates() {
    assert_eq!(concat("hello", "world"), "hello world");
    assert_eq!(concat("goodbye", "me"), "goodbye me");
}
