use std::fs;

fn main() {
    let mut args = std::env::args().skip(1);

    let key = args.next().expect("Key was not there"); // panick, but give error msg
    let value = args.next().unwrap(); // succeed or panick

    println!("The key is '{}' and the value is '{}'", key, value);

    let contents = format!("{}\t{}\n", key, value);

    let write_result = fs::write("kv.db", contents);

    match write_result {
        Ok(()) => println!("Successfully written to db!"),
        Err(e) => println!("Writing to db failed {}", e),
    }
}
