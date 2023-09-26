// Topic: External crates
//
// Requirements:
// * Display the current date and time
//
// Notes:
// * Use the `chrono` crate to work with time
// * (OPTIONAL) Read the documentation section `Formatting and Parsing`
//   for examples on how to create custom time formats

use chrono::prelude::*;

fn main() {
    let lt = Local::now();
    let dt = lt
        .format_localized("%A %e %B %Y, %T", Locale::nl_NL)
        .to_string();
    println!("the current date and time are {:?}", dt)
}
