use super::input;
use super::types::Bill;
use std::io;

use std::collections::HashMap;

pub fn add(bills: &mut HashMap<String, Bill>) {
    let mut name: io::Result<String>;
    let mut amount: io::Result<String>;
    let mut confirmation: io::Result<String>;

    loop {
        name = input::read_user_input(Some("Enter Bill Name"));

        match name {
            Err(_) => continue,
            Ok(_) => break,
        }
    }

    loop {
        amount = input::read_user_input(Some("Enter Bill Amount"));

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

    let bill = Bill::new(name.unwrap(), amount.unwrap().parse::<f32>().unwrap());

    loop {
        confirmation = input::read_user_input(Some(
            format!(
                "Name: {}, Amount: {}, is this correct? ([y]es/[n]o/[a]bort)",
                &bill.name, &bill.amount
            )
            .as_str(),
        ));

        match confirmation {
            Err(_) => continue,
            Ok(c) => {
                if c.to_lowercase() == "y" {
                    bills.insert(bill.name.clone(), bill);

                    _ = input::read_user_input(Some("Bill added..."));
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
