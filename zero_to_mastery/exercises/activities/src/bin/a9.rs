// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn main() {
    let (_x, y) = get_tuple();

    if y > 5 {
        println!("y {} > 5", y)
    } else if y < 5 {
        println!("y {} < 5", y)
    } else {
        println!("y {} = 5", y)
    }
}

fn get_tuple() -> (i32, i32) {
    let point = (2, 3);

    (point.0, point.1)
}
