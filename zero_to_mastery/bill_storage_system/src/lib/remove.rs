use super::{input, Bill};
use std::io;

pub fn remove(bills: &mut Vec<Bill>) {
    let mut name: io::Result<String>;
    let search_name: String;

    loop {
        name = input::read_user_input(Some("Enter Bill Name to Search"));

        match name {
            Err(_) => continue,
            Ok(search) => {
                search_name = search;
                break;
            }
        }
    }

    let index = bills.iter().position(|b| b.name == search_name);

    match index {
        Some(i) => {
            let bill = &bills[i];
            println!("Removing {:?} from the System", bill);

            bills.remove(i);
        }
        None => println!("Bill with name not found, please try again."),
    }
    _ = input::read_user_input(None);
}
