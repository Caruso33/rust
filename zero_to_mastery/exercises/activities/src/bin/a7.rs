// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Color {
    YELLOW,
    BLUE,
    RED,
    GREEN,
}

fn main() {
    let var = Color::BLUE;

    print_color(var);
}

fn print_color(color: Color) {
    match color {
        Color::YELLOW => println!("it's yellow"),
        Color::BLUE => println!("it's blue"),
        Color::RED => println!("it's red"),
        Color::GREEN => println!("it's green"),
    }
}
