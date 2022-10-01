// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Salty,
    Sweet,
    Bitter,
    Sour,
    Spicy,
}

struct Drink {
    flavor: Flavor,
    fluid: f32,
}

fn main() {
    let var = Drink {
        flavor: Flavor::Spicy,
        fluid: 5.3,
    };

    print_drink(var);
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Salty => println!("it's salty"),
        Flavor::Sweet => println!("it's sweet"),
        Flavor::Bitter => println!("it's bitter"),
        Flavor::Sour => println!("it's sour"),
        Flavor::Spicy => println!("it's spicy"),
    }
    println!("The drink has this much fluid: {}", drink.fluid);
}
