// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing

#[derive(Debug)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}

#[derive(Debug)]
struct ShoesColor(Color);
impl ShoesColor {
    fn new(color: Color) -> Self {
        ShoesColor(color)
    }
}

#[derive(Debug)]
struct ShirtColor(Color);
impl ShirtColor {
    fn new(color: Color) -> Self {
        ShirtColor(color)
    }
}

#[derive(Debug)]
struct PantsColor(Color);
impl PantsColor {
    fn new(color: Color) -> Self {
        PantsColor(color)
    }
}

fn print_new_types<T: std::fmt::Debug>(item: T) {
    println!("{:?}", item);
}

fn main() {
    let shirt_color = ShirtColor::new(Color::Gray);
    let pants_color = PantsColor::new(Color::Blue);
    let shoes_color = ShoesColor::new(Color::White);

    print_new_types(shirt_color);
    print_new_types(pants_color);
    print_new_types(shoes_color);
}
