// Topic: Generics & Structures
//
// Requirements:
// * Create a Vehicle structure that is generic over traits Body and Color
// * Create structures for vehicle bodies and vehicle colors and implement the
//   Body and Color traits for these structures
// * Implement a 'new' function for Vehicle that allows it to have any body
//   and any color
// * Create at least two different vehicles in the main function and print their
//   info
//
// Notes:
// * Examples of car bodies can be Truck, Car, Scooter
// * Examples of colors could be red, white, black
// * It is not necessary to have data fields or function implementations
//   for the vehicle bodies/colors

trait Body {
    fn print_body_info(&self);
}

trait Color {
    fn print_color_info(&self);
}

#[derive(Debug)]
struct Vehicle<B: Body, C: Color> {
    body: B,
    color: C,
}
impl<B: Body, C: Color> Vehicle<B, C> {
    fn new(body: B, color: C) -> Self {
        Vehicle { body, color }
    }
    fn print_info(&self) {
        self.print_body();
        self.print_color();
    }
    fn print_body(&self) {
        self.body.print_body_info();
    }
    fn print_color(&self) {
        self.color.print_color_info();
    }
}

// impl<B: Body, C: Color> Default for Vehicle<B, C> {
//     fn default() -> Self {
//         // Vehicle {
//         //     body: VehicleBody::Car,
//         //     color: VehicleColor::White,
//         // };
//         Vehicle::new(VehicleBody::Car, VehicleColor::White)
//     }
// }

#[derive(Debug)]
enum VehicleBody {
    Truck,
    Car,
    Scooter,
}
impl Body for VehicleBody {
    fn print_body_info(&self) {
        println!("{:?}", self);
    }
}

#[derive(Debug)]
enum VehicleColor {
    Red,
    White,
    Black,
}
impl Color for VehicleColor {
    fn print_color_info(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let truck1 = Vehicle::new(VehicleBody::Truck, VehicleColor::Red);
    let truck2 = Vehicle::new(VehicleBody::Truck, VehicleColor::White);
    let car1 = Vehicle::new(VehicleBody::Car, VehicleColor::Black);
    let car2 = Vehicle::new(VehicleBody::Car, VehicleColor::White);
    let scooter = Vehicle::new(VehicleBody::Scooter, VehicleColor::Black);

    truck1.print_info();
    truck2.print_info();
    car1.print_info();
    car2.print_info();
    scooter.print_info();
}
