// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Grocery(i32, i32);

fn main() {
    let grocery = Grocery(2, 20123);

    display_quantity(&grocery);
    display_number(&grocery);
}

fn display_quantity(grocery: &Grocery) {
    println!("{}", grocery.0);
}

fn display_number(grocery: &Grocery) {
    println!("{}", grocery.1);
}
