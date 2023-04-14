// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
struct Dimensions {
    x: i32,
    y: i32,
    z: i32
}

impl Dimensions {
    fn print(&self) {
        println!("The shipping box dimensions are: X: {:?}, Y: {:?}, Z: {:?}",
                          self.x, self.y, self.z);
    }
}

enum Color {
    Yellow,
    Blue,
    Red,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Yellow => println!("The shipping box color is: yellow"),
            Color::Blue => println!("The shipping box color is: blue"),
            Color::Red => println!("The shipping box color is: red"),
        }
    }
}

struct ShippingBox {
    dimensions: Dimensions,
    weight: i32,
    color: Color
}

impl ShippingBox {
    fn new(dimensions: Dimensions, weight: i32, color: Color) -> Self {
        ShippingBox {
           dimensions,
            weight,
            color,
        }
    }

    fn update_weight(&mut self, weight: i32) {
        self.weight = weight;
    }

    fn update_color(&mut self, color: Color) {
        self.color = color;
    }

    fn print(&self) {
        self.dimensions.print();
        println!("The shipping box weight is: {:?} grams", self.weight);
        self.color.print();
    }
}

fn main() {
    let dimensions = Dimensions {
        x: 10,
        y: 20,
        z: 40,
    };
    let weight= 100;
    let color = Color::Yellow;

    let my_shipping_box = ShippingBox::new(dimensions, weight, color);
    my_shipping_box.print();

    let mut blue_shipping_box = ShippingBox::new(Dimensions {
        x: 50,
        y: 14,
        z: 33,
    },
        222,
        Color::Blue,
    );

    blue_shipping_box.print();
    blue_shipping_box.update_color(Color::Red);
    blue_shipping_box.update_weight(44);
    blue_shipping_box.print();
}
