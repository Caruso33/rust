use super::{input, Bill};
use std::collections::HashMap;

pub fn view(bills: &mut HashMap<String, Bill>) {
    println!("\nCurrent Bills:\n");

    let mut total_amount: f32 = 0.0;

    for (_key, bill) in bills.iter() {
        total_amount += bill.amount;

        println!("{:?}", bill);
    }

    println!("Total amount billed: {:?}", total_amount);

    _ = input::read_user_input(None);
}
