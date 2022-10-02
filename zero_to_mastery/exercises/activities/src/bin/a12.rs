// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum Color {
    Blue,
    Green,
}

struct ShippingBox {
    dimensions: (f32, f32, f32),
    weight: f32,
    color: Color,
}

impl ShippingBox {
    fn new(dimensions: (f32, f32, f32), weight: f32, color: Color) -> Self {
        ShippingBox {
            dimensions,
            weight,
            color,
        }
    }

    fn print_characteristics(&self) {
        let color_print = match self.color {
            Color::Blue => "blue",
            Color::Green => "green",
        };

        println!(
            "The dimension is {}x{}x{}, the weight {}, the color {}",
            self.dimensions.0, self.dimensions.1, self.dimensions.2, self.weight, color_print
        );
    }
}

fn main() {
    let shippingBox = ShippingBox::new((22.3, 51.15, 13.1113), 15.59, Color::Green);

    shippingBox.print_characteristics();
}
