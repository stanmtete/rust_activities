// Topic: HashMap
//
// Requirements
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
// * 5 Chairs 
// * 3 Beds 
// * 2 Tables 
// * 0 Cauches
// * Print the total number of item in stock
//
// Notes
// * Use a HashMap for the furniture store stock 

use std::collections::HashMap;

// * Use a HashMap for the furniture store stock 
struct Furniture {
    name: String,
}

impl Furniture {
    fn new(name: String) -> Self {
        Self { name }
    }
}
fn main() {
    let mut furnitures = HashMap::new();

    furnitures.insert(5, Furniture::new("Chairs".to_owned()));
    furnitures.insert(3, Furniture::new("Beds".to_owned()));
    furnitures.insert(2, Furniture::new("Tables".to_owned()));
    furnitures.insert(0, Furniture::new("Cauches".to_owned()));

    //Print out the furnitures using: for each loop
    println!("Furniture Store Stock");

    for (number, furniture) in furnitures.iter() {
        match number {
            0 => println!("Number: out of stock"),
            others => println!("Number: {:?}", others),
        }
        println!("Name: {:?}", furniture.name);
    }

}