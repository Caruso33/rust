mod pig_latin;

fn main() {
    // give_me_mean();
    // give_me_pig_latin();
    // pig_latin::main();
    hash_map_vectors();

    fn give_me_mean() {
        // Given a list of integers, use a vector and return the mean (the average value),
        // median (when sorted, the value in the middle position),
        // and mode (the value that occurs most often; a hash map will be helpful here) of the list.
        let integers = vec![1, 3, 5, 6, 7, 8, 9, 9, 42342, 342, 234];

        let mut sum = 0;

        for n in &integers {
            sum += n;
        }

        let mean: f64 = (sum / integers.len()) as f64;

        println!("sum: {}", sum);
        println!("len: {}", integers.len());
        println!("mean: {}", mean);
    }

    // Convert strings to pig latin. The first consonant of each word
    // is moved to the end of the word and “ay” is added,
    // so “first” becomes “irst-fay.” Words that start with a vowel
    // have “hay” added to the end instead (“apple” becomes “apple-hay”).
    // Keep in mind the details about UTF-8 encoding!
    fn give_me_pig_latin() {
        let st = String::from("All apples fall from above tree.");
        let sp = st.split(" ");

        let mut result = Vec::new();

        for s in sp {
            let part = s.replace(".", "");

            let first = part.chars().nth(0).unwrap().to_ascii_lowercase();

            if match first {
                'a' | 'e' | 'i' | 'o' | 'u' => true,
                _ => false,
            } {
                result.push(format!("{}-hay", part));
            } else {
                result.push(format!("{}-{}ay", &part[first.len_utf8()..], first));
            }
        }

        let pig_latin = result.join(" ");

        println!("pig latin: {}", pig_latin);
    }

    // Using a hash map and vectors, create a text interface to allow a user to
    // add employee names to a department in a company. For example,
    // “Add Sally to Engineering” or “Add Amir to Sales.”
    // Then let the user retrieve a list of all people in a department
    // or all people in the company by department, sorted alphabetically.
    use std::collections::HashMap;
    use std::io::stdin;

    #[derive(Debug, PartialEq)]
    pub enum Error {
        EmptyString,
    }

    fn hash_map_vectors() {
        // let employees = HashMap::new();

        fn evaluate_input(inp: &str) -> () {// Result<String, Error> {
            let commands = inp.split_whitespace();
            // let mut cmd;
            // let mut employee;
            // let mut department;

            for (i, c) in commands.enumerate() {
                println!("{}, {}", i, c)
                // match i {
                //     1 => employee = c,
                //     3 => department = c,
                //     _ => continue,
                // }
            }
            // return String::from("lalla")
            // match cmd {
            //     'Add' =>
            // }
        }

        // loop till `exit` found
        loop {
            let input = {
                let mut buf = String::new();
                stdin().read_line(&mut buf).expect("Failed to read line");
                buf.trim().to_string()
            };
            if input.to_lowercase() == "exit" {
                break;
            } else {
                evaluate_input(&input)
                // match evaluate_input(&input) {
                //     Ok(out) => println!("{}", out),
                //     Err(Error::EmptyString) => println!("ZERO LENGTH STRINGS NOT ALLOWED"),
                // }
            }
        }
    }
}
