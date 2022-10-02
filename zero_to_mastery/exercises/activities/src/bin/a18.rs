// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

struct Customer {
    age: i32,
}

impl Customer {
    fn can_purchase_restricted(&self) -> Result<(), &str> {
        if self.age >= 21 {
            Ok(())
        } else {
            Err("Age is under 21")
        }
    }
}

fn main() {
    let ali = Customer { age: 15 };
    let bob = Customer { age: 51 };

    check_for_restriction(&ali);
    check_for_restriction(&bob);
}

fn check_for_restriction(c: &Customer) {
    match c.can_purchase_restricted() {
        Ok(_) => println!("All good, go ahead"),
        Err(e) => println!("{:?}", e),
    }
}
