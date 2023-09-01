// Project 1: Interactive bill manager
//
// User stories:
// * L1: I want to add bills, including the name and amount owed.
// * L1: I want to view existing bills.
// * L2: I want to remove bills.
// * L3: I want to edit existing bills.
// * L3: I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at level 1, but a
//   hashmap will be easier to work with at levels 2 and 3.
// * Create a function just for retrieving user input, and reuse it
//   throughout your program.
// * Create your program starting at level 1. Once finished, advance to the
//   next level.

use std::io;

struct Bill {
    name: String,
    amount: i32,
}

struct Bills {
    inner: Vec<Bill>,
}

impl Bills {
    fn new() -> Self {
        Self { inner: vec![] }
    }

    fn add(&mut self) -> () {
        let new_bill = add_bill();
        self.inner.push(new_bill);
        println!("Your new bill has successfully been added!🎉");
        match self.inner.last() {
            None => println!("Something went wrong. No bills added!"),
            Some(bill) => println!("Bill name: {} and amount: {}", bill.name, bill.amount),
        }
    }

    fn view_all(&self) {
        match self.inner.is_empty() {
            true => println!("No bills available"),
            false => {
                for bill in &self.inner {
                    println!("name: {} amount: {}", bill.name, bill.amount)
                }
            }
        }
    }
}

enum MenuOption {
    Add,
    View,
    Invalid,
    Exit,
}

// std::io operations
fn get_input() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("please enter a menu option");
    buffer
}

// menu operations
fn display_menu() {
    println!("Please select your option:");
    println!("1 - Add a new bill");
    println!("2 - View bills");
    println!("0 - Exit menu");
}

fn get_menu_choice() -> MenuOption {
    let input = get_input();
    match input.trim() {
        "1" => MenuOption::Add,
        "2" => MenuOption::View,
        "0" => MenuOption::Exit,
        _ => MenuOption::Invalid,
    }
}

fn add_bill() -> Bill {
    println!("Add a new bill!");
    println!("Please add name:");
    let name = get_input();
    println!("Please add an amount:");
    let amount = get_input().trim().parse::<i32>().unwrap();
    Bill { name, amount }
}

fn main() {
    let mut bills = Bills::new();
    loop {
        display_menu();
        match get_menu_choice() {
            MenuOption::Add => bills.add(),
            MenuOption::View => bills.view_all(),
            MenuOption::Invalid => println!("Please choose a valid menu option"),
            MenuOption::Exit => break,
        };
    }
}
