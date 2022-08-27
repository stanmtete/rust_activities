// In these tutorial demostrates the standard library used in rust

fn main() {
    let numbers = vec![1,2,3];

    match numbers.is_empty() {
        true => println!("No elements"),
        false => println!("Has elements"),
    }
}