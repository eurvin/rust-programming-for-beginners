// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined States
// * The States represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the States, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the State entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

use std::io;

enum State {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}

fn fmt_string(s: String) -> String {
    let mut fmtd_s = s.chars();
    match fmtd_s.next() {
        Some(c) => c.to_uppercase().collect::<String>() + fmtd_s.as_str(),
        None => String::new(),
    }
}

fn handle_input(user_input: String) -> Result<State, ()> {
    match user_input.as_str() {
        "Off" => Ok(State::Off),
        "Sleep" => Ok(State::Sleep),
        "Reboot" => Ok(State::Reboot),
        "Shutdown" => Ok(State::Shutdown),
        "Hibernate" => Ok(State::Hibernate),
        _ => Err(()),
    }
}

fn print_state(state: State) {
    match state {
        State::Off => println!("Turning off!"),
        State::Sleep => println!("Turning to sleep!"),
        State::Reboot => println!("Rebooting!"),
        State::Shutdown => println!("Shutting down!"),
        State::Hibernate => println!("Hibernating!"),
    }
}

fn main() {
    let input_result = get_input();
    let fmt_input = match input_result {
        Ok(user_input) => fmt_string(user_input),
        Err(e) => e.to_string(),
    };
    let get_state = handle_input(fmt_input);
    let state = get_state;
    if state.is_ok() {
        print_state(state.unwrap());
    }
}
