use std::collections::HashMap;
use std::fs;

fn main() {
    let mut args = std::env::args().skip(1);

    let key = args.next().expect("Key was not there"); // panick, but give error msg
    let value = args.next().unwrap(); // succeed or panick

    println!("The key is '{}' and the value is '{}'", key, value);

    // match write_result {
    //     Ok(()) => println!("Successfully written to db!"),
    //     Err(e) => println!("Writing to db failed {}", e),
    // }

    let mut database = Database::new().expect("Database::new() crashed!");
    database.insert(key.clone(), value.clone());
    // or Database::insert(database, key,)
    database.insert(key.to_uppercase(), value.clone());

    // database.flush().unwrap()
    match database.flush() {
        Ok(()) => println!("Yay"),
        Err(err) => println!("Oh nos, {}", err),
    }
}

struct Database {
    map: HashMap<String, String>,
    is_flushed: bool,
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

        Ok(Database {
            map,
            is_flushed: false,
        })
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    // fn flush(self) -> Result<(), std::io::Error> {
    fn flush(&mut self) -> std::io::Result<()> {
        self.is_flushed = true;
        do_flush(&self)
    }
}

// save database on programm end automagically
impl Drop for Database {
    fn drop(&mut self) {
        if !self.is_flushed {
            let _ = do_flush(self);
        }
    }
}

fn do_flush(database: &Database) -> std::io::Result<()> {
    let mut contents = String::new();
    // for pairs in self.map {
    // let kvpair = format!("{}\t{}\n", pairs.0, pairs.1);

    for (key, value) in &database.map {
        contents.push_str(key);
        contents.push('\t');
        contents.push_str(value);
        contents.push('\n');

        // let kvpair = format!("{}\t{}\n", key, value);
        // contents.push_str(&kvpair);
    }
    std::fs::write("kv.db", contents)

    // todo!("Finish this method") // compiler, pretend rest of the function is okay
}
