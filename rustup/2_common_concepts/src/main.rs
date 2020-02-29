use std::io;

fn main() {
    vars_n_immutability();
    data_types();
    functions();
    control_flow();

    practice();
}

fn vars_n_immutability() {
    mutability();
    shadowing();
    fn mutability() {
        let mut x = 5;
        println!("The value of x is: {}", x);
        x = 6;
        println!("The value of x is: {}", x);
    }

    fn shadowing() {
        let x = 5;

        let x = x + 1;

        let x = x * 2;

        println!("The value of x is: {}", x);
        // different type allowed
        let spaces = "   ";
        let spaces = spaces.len();

        // would error
        /* let mut spaces = "   ";
        spaces = spaces.len(); */

        println!("The value of spaces is: {}", spaces);
    }
}

fn data_types() {
    scalars();
    numeric_operations();
    compounds();

    fn scalars() {
        // throws without type
        // let guess: u32 = "42".parse().expect("Not a number!");

        /*
        Decimal	98_222
        Hex	0xff
        Octal	0o77
        Binary	0b1111_0000
        Byte (u8 only)	b'A'
         */

        // floats
        let x = 2.0; // f64

        let y: f32 = 3.0; // f32

        // bool
        let t = true;

        let f: bool = false; // with explicit type annotation

        // char
        let c = 'z';
        let z = 'ℤ';
        let heart_eyed_cat = '😻';

        println!("{} {} {} {} {} {} {}", x, y, t, f, c, z, heart_eyed_cat);
    }

    fn numeric_operations() {
        // addition
        let sum = 5 + 10;

        // subtraction
        let difference = 95.5 - 4.3;

        // multiplication
        let product = 4 * 30;

        // division
        let quotient = 56.7 / 32.2;

        // remainder
        let remainder = 43 % 5;

        println!(
            "{} {} {} {} {}",
            sum, difference, product, quotient, remainder
        );
    }

    fn compounds() {
        // tuples, have fixed size
        let tup: (i32, f64, u8) = (500, 6.4, 1);

        let five_hundred = tup.0;
        let (_, y, _) = tup;

        println!("{} {}", five_hundred, y);

        // array, same type, fixed size
        let a: [i32; 5] = [1, 2, 3, 4, 5];
        let b = [4; 10]; // number 4, 10 times

        let months = [
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December",
        ];

        println!("{}{} {}", a[0], b[9], months[9]);
    }
}

fn functions() {
    another_function(5);
    statement_vs_expression();
    let x = plus_one(5);
    println!("5 plus_one is: {}", x);

    fn another_function(x: i128) {
        // needs type declaration of arguments
        println!("The value of x is: {}", x);
    }

    fn statement_vs_expression() {
        // statement, not assignable
        // let x = (let y = 6);

        // entire {} block is expression
        let y = {
            let x = 3;
            x + 1 // with ';' it would be a statement
        };

        println!("The value of y is: {}", y);
    }

    // declared return type
    fn plus_one(x: i32) -> i32 {
        x + 1 // implicit return
    }
}

fn control_flow() {
    if_statements();
    loops();

    fn if_statements() {
        let number = 3;

        // if number { ... } doesn't work as number is no bool

        if number == 0 {
            println!("number is zero");
        } else if number < 5 {
            println!("condition was true");
        } else {
            println!("condition was false");
        }

        let condition = true;
        // if is a expression!
        let number = if condition { 5 } else { 6 };

        println!("The value of number is: {}", number);
    }

    fn loops() {
        // potentially infinite-loop
        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2; // return value, works with or without ';'
            }
        }; // statements: assigning to result, end with ';'

        println!("The result is: {}", result);

        // while-loop
        let mut number = 3;

        while number != 0 {
            println!("{}!", number);

            number -= 1;
        }

        println!("LIFTOFF!!");

        // iterating with for-loop
        let a = [10, 20, 30, 40, 50];

        for element in a.iter() {
            println!("the value is: {}", element);
        }

        // countdown with for-loop
        for number in (1..4).rev() {
            println!("{}!", number);
        }
        println!("LIFTOFF!!!");
    }
}

fn practice() {
    let (temp_degrees, temp_type) = read_stdin();
    let conversion;

    if temp_type.starts_with('c') {
        conversion = convert_c_to_f(temp_degrees);
    } else {
        conversion = convert_f_to_c(temp_degrees);
    }

    println!(
        "Your conversion is: {}°{}",
        conversion.round(),
        if temp_type.starts_with('c') { "F" } else { "C" }
    );

    fn read_stdin() -> (f64, String) {
        let mut temp_degrees: f64;
        let mut temp_type: String;

        let result = loop {
            println!("Please input your temperatur degrees");
            let mut degrees_input = String::new();

            io::stdin()
                .read_line(&mut degrees_input)
                .expect("Failed to read line");

            temp_degrees = match degrees_input.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            println!("Please input your temperatur type as f or c");

            let mut type_input = String::new();

            io::stdin()
                .read_line(&mut type_input)
                .expect("Failed to read line");

            temp_type = type_input.trim().to_lowercase().to_string();

            if !temp_type.starts_with('c') && !temp_type.starts_with('f') {
                continue;
            }
            break (temp_degrees, temp_type);
        };

        result
    }

    fn convert_c_to_f(c: f64) -> f64 {
        c * 1.8 + 32.
    }

    fn convert_f_to_c(f: f64) -> f64 {
        f - 32f64 / 1.8
    }
}
