// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

trait Perimeter {
    fn calculate_perimeter(&self) -> i32;
}

struct Square(i32);
struct Triangle(i32, i32, i32);

impl Perimeter for Square {
    fn calculate_perimeter(&self) -> i32 {
        self.0 * 4
    }
}

impl Perimeter for Triangle {
    fn calculate_perimeter(&self) -> i32 {
        self.0 + self.1 + self.2
    }
}

fn main() {
    let square = Square(5);

    let triangle = Triangle(2, 3, 4);

    assert_eq!(square.calculate_perimeter(), 20);
    assert_eq!(triangle.calculate_perimeter(), 9);
}
