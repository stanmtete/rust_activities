//Topic: Implementing functionality with the impl keyword 
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes
// * Use a struct to encapsulate the box characteristics 
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box 
// * implement functionality on the box struct to print the characteristics 

// * Use an enum for the box color
#[allow(dead_code)]
enum Color {
    Blue,
    Black,
    Red,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Black => println!("Color: black"),
            Color::Blue => println!("Color: blue"),
            Color::Red => println!("Color: red"),
        }
    }
}

// * Dimensions struct and its implementation
struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl  Dimensions {
    fn print(&self) {
        println!("Dimensions\n\tWidth: {:?}\n\tHeight: {:?}\n\tDepth: {:?}", self.width, self.height, self.depth);
    }
}

// * Use a struct to encapsulate the box characteristics 
struct ShippingBox {
    dimensions: Dimensions,
    weight: f64,
    color: Color,
}

// * Implement functionality on the box struct to create a new box 
impl ShippingBox {
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self { 
            weight, 
            dimensions, 
            color,
         }
    }

    fn print(&self) {
        println!("Weight: {:?}", self.weight);
        self.dimensions.print();
        self.color.print();
    }
}
fn main() {
   let small_dimensions = Dimensions {
    width: 10.0,
    height: 12.0,
    depth: 8.0,
   };

   let small_box = ShippingBox::new(5.0, Color::Black, small_dimensions);
   small_box.print();
}