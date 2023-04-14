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

// * Use an enum for the tickets with data associated with each variant
#[derive(Debug)]
enum Ticket {
    Backstage(i32, String),
    Standard(i32),
    Vip(i32, String),
}

fn main() {
    // * Create one of each ticket and place into a vector
    let backstage = Ticket::Backstage(50, "Johnny".to_owned());
    let vip = Ticket::Vip(25, "alice".to_owned());
    let std = Ticket::Standard(10);
    let tickets = vec![std, backstage, vip];

    for ticket in tickets {
        // * Use a match expression while iterating the vector to print the ticket info
        match ticket {
            Ticket::Backstage(amount, name) => println!(
                "The backstage ticket costs {:?}, and is assigned to {:?}",
                amount, name
            ),
            Ticket::Standard(amount) => println!("The standard ticket costs {:?}", amount),
            Ticket::Vip(amount, name) => println!(
                "The VIP ticket costs {:?}, and is assigned to {:?}",
                amount, name
            ),
        }
    }
}
