// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn print_message(is_big_number: bool) {
    match is_big_number {
        true => println!("its big"),
        false => println!("its small"),
    };
}

fn main() {
    let number_size = 100;
    let is_big_number = number_size > 100;

    // let is_big_number = if number_size > 100 {
    //     true
    // } else {
    //     false
    // };

    print_message(is_big_number)
}
