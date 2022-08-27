//TOPIC: BASIC ARTHIMETIC 
// Display the result of the two numbers

//function to add the two numbers
fn sum(a: i32, b: i32) -> i32 {
    a+b
}

fn main() {

    let result = sum(10,20);
    println!("{:?}", result)
}