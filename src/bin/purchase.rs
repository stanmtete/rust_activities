//This is the short program that check the age of the user and allow to purchase or cannot purchase


fn main() {
    //create a variable to hold the user age
    let custom_age = 15;


    //conditions to check out if the custome is eligible to purchase or not
    if custom_age >= 18 {
        println!("Can purchase");
    }else {
        println!("Cannot purchase");
    }
}