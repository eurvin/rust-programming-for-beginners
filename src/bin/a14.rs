// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
struct Person {
    age: i8,
    name: String,
    color: String,
}

impl Person {
    fn new(age: i8, name: String, color: String) -> Self {
        Self {
            age,
            name,
            color,
        }
    }
    // * The name and colors should be printed using a function
    fn print(&self) {
        println!("{:?}'s favorite color is {:?}", self.name, self.color)
    }
}

fn main() {
    // * Create and store at least 3 people in a vector
    let john = Person::new(9, "John".to_owned(), "yellow".to_owned());
    let bob = Person::new(10, String::from("Bob"), String::from("blue"));
    let alice = Person::new(12, "Alice".to_owned(), "red".to_owned());
    let my_people = vec![john, bob, alice];
    for person in my_people {
        let ten_and_under = person.age <= 10;
        match ten_and_under {
            true => person.print(),
            false => (),
        }
    }
}
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function