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

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io;

struct Bill {
    name: String,
    amount: i32,
}

struct Bills {
    list: HashMap<String, Bill>,
}

impl Bills {
    fn new() -> Self {
        Self {
            list: HashMap::new(),
        }
    }

    fn add(&mut self, bill: Bill) -> Result<String, ()> {
        match self.list.entry(bill.name.to_owned()) {
            Entry::Occupied(..) => Err(()),
            Entry::Vacant(entry) => {
                let name = bill.name.to_owned();
                entry.insert(bill);
                Ok(name)
            }
        }
    }

    fn edit(&mut self, bill: Bill) {
        self.list.entry(bill.name.to_owned()).or_insert(bill);
    }
}

enum MenuOption {
    Add,
    View,
    ViewAll,
    Edit,
    Delete,
    Invalid,
    Exit,
}

#[derive(Debug)]
enum BillsError {
    Empty,
    AlreadyExists,
    Invalid,
}

// std::io operations
fn get_input() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("please enter a menu option");
    buffer
}

// Errors
fn print_errors(error: Errors) {
    match error {
        BillsError::Empty => println!("Something went wrong. No bills found!"),
        BillsError::AlreadyExists => println!("A bill with this name already exists!"),
        BillsError::Invalid => println!("Please choose a valid menu option"),
    }
}

// menu operations
fn display_menu() {
    println!("Please select your option:");
    println!("1 - Add a new bill");
    println!("2 - View bill");
    println!("3 - View all bills");
    println!("4 - Edit bills");
    println!("5 - Delete bills");
    println!("0 - Exit menu");
}

fn get_menu_choice() -> MenuOption {
    let input = get_input();
    match input.trim() {
        "1" => MenuOption::Add,
        "2" => MenuOption::View,
        "3" => MenuOption::ViewAll,
        "4" => MenuOption::Edit,
        "5" => MenuOption::Delete,
        "0" => MenuOption::Exit,
        _ => MenuOption::Invalid,
    }
}

// bill operations
fn print_bill(bills: &Bills, name: &String) {
    match bills.list.get(&name) {
        Some(bill) => println!("Bill name: {} and amount: {}", bill.name, bill.amount),
        None => print_errors(BillsError::Empty),
    }
}

fn add_bill(bills: &mut Bills) {
    println!("Add a new bill.\n");
    println!("Please add name:");
    let name = get_input();
    match bills.list.contains_key(&name) {
        true => print_errors(BillsError::AlreadyExists),
        false => {
            println!("Please add an amount:");
            let amount = get_input().trim().parse::<i32>().unwrap();
            let new_bill = Bill { name, amount };
            match bills.add(new_bill) {
                Err(..) => print_errors(BillsError::AlreadyExists),
                Ok(name) => {
                    println!("Your new bill has successfully been added!🎉\n");
                    println!("You have added the following new bill: \n");
                    print_bill(&bills, &name)
                }
            }
        }
    }
}

fn view_bill(bills: &Bills) {
    match bills.list.is_empty() {
        true => print_errors(BillsError::Empty),
        false => {
            println!("Please select a bill (name)");
            let name = get_input();
            print_bill(&bills, &name)
        }
    }
}

fn view_all_bills(bills: &Bills) {
    match bills.list.is_empty() {
        true => print_errors(BillsError::Empty),
        false => {
            println!("We found the following bills:");
            for (_, bill) in bills.list.iter() {
                println!("name: {} amount: {}", bill.name, bill.amount)
            }
        }
    }
}

fn edit_bill(bills: &mut Bills) {
    match bills.list.is_empty() {
        true => print_errors(BillsError::Empty),
        false => {
            println!("Please select a bill (name):");
            view_all_bills(&bills);
            let name = get_input();
            println!("Please add a new amount:");
            let amount = get_input().trim().parse::<i32>().unwrap();
            bills.edit(Bill { name, amount })
        }
    }
}

fn delete_bill(bills: &mut Bills) {
    match bills.list.is_empty() {
        true => print_errors(BillsError::Empty),
        false => {
            println!("Please select a bill (name):");
            view_all_bills(&bills);
            let name = get_input();
            bills.list.remove(name.as_str());
            println!("bill: {} deleted:", name);
        }
    }
}

fn main() {
    let mut bills = Bills::new();
    loop {
        display_menu();
        match get_menu_choice() {
            MenuOption::Add => add_bill(&mut bills),
            MenuOption::View => view_bill(&bills),
            MenuOption::ViewAll => view_all_bills(&bills),
            MenuOption::Edit => edit_bill(&mut bills),
            MenuOption::Delete => delete_bill(&mut bills),
            MenuOption::Exit => break,
            MenuOption::Invalid => print_errors(BillsError::Invalid),
        };
    }
}
