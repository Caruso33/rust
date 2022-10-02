// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age: i16,
    name: String,
    color: String,
}

fn main() {
    let mut people: Vec<Person> = Vec::new();

    let ali = Person {
        age: 9,
        name: "Ali".to_string(),
        color: "green".to_string(),
    };
    let bob = Person {
        age: 51,
        name: "Bob".to_string(),
        color: "blue".to_string(),
    };
    let charlie = Person {
        age: 28,
        name: "Charlie".to_string(),
        color: "yellow".to_string(),
    };

    people.push(ali);
    people.push(bob);
    people.push(charlie);

    for p in &people {
        if p.age <= 10 {
            println!("Name: {:?}, Color: {:?}", p.name, p.color);
        }
    }
}
