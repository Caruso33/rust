use std::collections::HashMap;
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

    let database = Database::new().expect("Database::new() crashed!");
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        // let contents = match fs::read_to_string("kv.db") {
        //     Ok(c) => c,
        //     Err(error) => {
        //         return Err(error);
        //     }
        // };

        let mut map = HashMap::new();
        let contents = fs::read_to_string("kv.db")?;

        for line in contents.lines() {
            let mut chunks = line.splitn(2, '\t');

            let key = chunks.next().expect("No key!");
            let value = chunks.next().expect("No value!");

            map.insert(key.to_string(), value.to_owned());
        }

        Ok(Database { map })
    }
}
