// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

fn main() {
    let mut furniture = HashMap::new();
    let mut total_number_items: i32 = 0;

    furniture.insert("Chairs", 5);
    furniture.insert("Beds", 3);
    furniture.insert("Tables", 2);
    furniture.insert("Couches", 0);

    // for (name, stock) in furniture { // moves

    // borrows
    for (name, stock) in furniture.iter() {
        if stock == &0 {
            println!("out of stock {}", name);
        }
        total_number_items += stock;
    }
    println!("total number of items {}", total_number_items);
}
