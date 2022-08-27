//Topic: Working with an enum


//An enum of colors
#[allow(dead_code)]
enum Colors {
    Blue,
    Black,
    Brown,
    Red
}


//Custom function to print out the colors
fn favorite_color(col: Colors) {
    match col {
        Colors::Black => println!("Black"),
        Colors::Blue => println!("Blue"),
        Colors::Brown => println!("Brown"),
        Colors::Red => println!("Red"),
    }
}

fn main() {
    favorite_color(Colors::Black);
}