use super::{input, Bill};

pub fn view(bills: &mut Vec<Bill>) {
    println!("\nCurrent Bills:\n");

    let mut total_amount: f32 = 0.0;

    for bill in bills {
        total_amount += bill.amount;

        println!("{:?}", bill);
    }

    println!("Total amount billed: {:?}", total_amount);

    _ = input::read_user_input(None);
}
