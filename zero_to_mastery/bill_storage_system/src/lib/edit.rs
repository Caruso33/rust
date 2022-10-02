use super::input;
use super::types::Bill;
use collections::HashMap;
use std::{collections, io};

pub fn edit(bills: &mut HashMap<String, Bill>) {
    let mut name: io::Result<String>;
    let mut search_name: String;

    let bill: &mut Bill;

    loop {
        name = input::read_user_input(Some("Enter Bill Name to Search"));

        match name {
            Err(_) => continue,
            Ok(search) => {
                search_name = search;
            }
        }

        let bill_opt: Option<&mut Bill> = bills.get_mut(&search_name.clone());

        match bill_opt {
            Some(b) => {
                println!("Found {:?} in the System", &b);

                bill = b;

                break;
            }
            None => {
                println!("Bill with name not found, please try again.");
                continue;
            }
        }
    }

    let mut name: io::Result<String>;
    let mut amount: io::Result<String>;
    let mut confirmation: io::Result<String>;

    loop {
        name = input::read_user_input(Some("Enter New Bill Name"));

        match name {
            Err(_) => continue,
            Ok(_) => break,
        }
    }

    loop {
        amount = input::read_user_input(Some("Enter New Bill Amount"));

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
        confirmation = input::read_user_input(Some(
            format!(
                "New Name: {}, New Amount: {}, is this correct? ([y]es/[n]o/[a]bort)",
                name.as_ref().unwrap(),
                amount.as_ref().unwrap()
            )
            .as_str(),
        ));

        match confirmation {
            Err(_) => continue,
            Ok(c) => {
                if c.to_lowercase() == "y" {
                    bill.name = name.unwrap();
                    bill.amount = amount.unwrap().parse::<f32>().unwrap();

                    _ = input::read_user_input(Some("Bill edited..."));
                    return;
                } else if c.to_lowercase() == "n" {
                    edit(bills);
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
