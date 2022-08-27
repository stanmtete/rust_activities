//Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item 
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity
// * Create a function to dispaly the id number

// * Use a struct for the grocery item 
struct Grocery {
    quantity: i32,
    id: i32,
}

// * Create a function to display the quantity
fn display_quantity(grocery: &Grocery) {
    println!("Quantity: {:?}", grocery.quantity);
}

// * Create a function to dispaly the id number
fn dispaly_id(grocery: &Grocery) {
    println!("Id number: {:?}", grocery.id);
}


fn main() {
    let item1 = Grocery {
        quantity: 20,
        id: 200,
    };

    display_quantity(&item1);
    dispaly_id(&item1);
}
