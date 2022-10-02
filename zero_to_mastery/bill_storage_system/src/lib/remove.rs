use super::{input, Bill};
use collections::HashMap;
use std::{collections, io};

pub fn remove(bills: &mut HashMap<String, Bill>) {
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

    let bill = bills.remove(&search_name);

    match bill {
        Some(b) => {
            println!("removed {:?} from the System", b);
        }
        None => println!("Bill with name not found, please try again."),
    }
    _ = input::read_user_input(None);
}
