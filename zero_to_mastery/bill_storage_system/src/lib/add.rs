// mod p1_core;
// mod p1_input;

use super::types::Bill;
use super::input::read_user_input;
use std::io;

pub fn add(bills: &mut Vec<Bill>) {
    let mut name: io::Result<String>;
    let mut amount: io::Result<String>;
    let mut confirmation: io::Result<String>;

    loop {
        name = read_user_input(Some("Enter Bill Name"));

        match name {
            Err(_) => continue,
            Ok(_) => break,
        }
    }

    loop {
        amount = read_user_input(Some("Enter Bill Amount"));

        if amount.is_err() {
            continue;
        }

        match &amount {
            Err(_) => continue,
            Ok(a) => {
                let float = a.parse::<f32>();
                match float {
                    Ok(_) => break,
                    Err(_) => continue,
                }
            }
        }
    }

    loop {
        confirmation = read_user_input(Some(
            format!(
                "Name: {}, Amount: {}, is this correct? ([y]es/[n]o/[a]bort)",
                name.as_ref().unwrap(),
                amount.as_ref().unwrap()
            )
            .as_str(),
        ));

        match confirmation {
            Err(_) => continue,
            Ok(c) => {
                if c.to_lowercase() == "y" {
                    let bill = Bill::new(name.unwrap(), amount.unwrap().parse::<f32>().unwrap());
                    bills.push(bill);
                    _ = read_user_input(Some("Bill added..."));
                    return;
                } else if c.to_lowercase() == "n" {
                    add(bills);
                    return;
                } else if c.to_lowercase() == "a" {
                    return;
                } else {
                    continue;
                }
            }
        }
    }
}
