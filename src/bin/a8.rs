// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

// * Use an enum to create different flavors of drinks
#[derive(Debug)]
enum Flavor {
    Strawberry,
    Lemon,
    Cherry,
}

// * Use a struct to store drink flavor and fluid ounce information
struct Drink {
    flavor: Flavor,
    fluid_ml: u8,
}

// * Use a function to print out the drink flavor and ounces
fn print_drink(my_drink: Drink) {
    // * Use a match expression to print the drink flavor
    match my_drink.flavor {
        Flavor::Cherry => println!("Your drink flavor is {:?} and contains {:?} ml", my_drink.flavor, my_drink.fluid_ml),
        Flavor::Lemon => println!("Your drink flavor is {:?} and contains {:?} ml", my_drink.flavor, my_drink.fluid_ml),
        Flavor::Strawberry => println!("Your drink flavor is {:?} and contains {:?} ml", my_drink.flavor, my_drink.fluid_ml),
    }
}

// * Use a match expression to print the drink flavor

fn main() {
    let my_drink = Drink {
        flavor: Flavor::Strawberry,
        fluid_ml: 250,
    };

    print_drink(my_drink)
}
