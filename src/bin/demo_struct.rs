
//struct 
struct GroceryItem {
    stock: i32,
    price: i64,
}

fn main() {
    let cereal = GroceryItem {
        stock: 10,
        price: 5,
    };

    println!("Stock {:?}", cereal.stock);
    println!("Price {:?}", cereal.price);
}