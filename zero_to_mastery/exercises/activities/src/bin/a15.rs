// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

#[derive(Debug)]
enum Ticket {
    Backstage(f32, String),
    Vip(f32, String),
    Standard(f32),
}

fn main() {
    let ticket1 = Ticket::Backstage(2150.13, "Ali".to_string());
    let ticket2 = Ticket::Vip(2150.13, "Bob".to_string());
    let ticket3 = Ticket::Standard(2150.13);

    let mut tickets = Vec::new();

    tickets.push(ticket1);
    tickets.push(ticket2);
    tickets.push(ticket3);

    for t in tickets {
        match t {
            Ticket::Standard(price) => println!("Standard Price: {:?}", price),
            Ticket::Backstage(price, holder) => {
                println!("Backstage Price: {:?}, Name: {:?}", price, holder)
            }
            Ticket::Vip(price, holder) => println!("Vip Price: {:?}, Name: {:?}", price, holder),
        }
    }
}
