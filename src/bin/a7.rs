// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

// * Use an enum with color names as variants

enum Color {
    White,
    Black,
    Yellow,
    Red,
    Blue
}

fn print_color(color: Color) {
    // * Use a match expression to determine which color
    //   name to print
    match color {
        Color::White => println!("White"),
        Color::Black => println!("Black"),
        Color::Yellow => println!("Yellow"),
        Color::Red => println!("Red"),
        Color::Blue => println!("Blue"),
    }
}

fn main() {
    let color = Color::Red;
    print_color(color)
}
