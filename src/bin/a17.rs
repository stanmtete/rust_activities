// Topic: Browsing standard library documentation
//
// Requirements:
// * Print a string in lowercase and uppercase 
//
// Notes: 
// * Utilize standard library functionality to transform the string to lowercase and uppercase 
// * Use 'rustup doc' in a terminal to open the standard library and navigate to the API documentation section
// * Search for functionality to transform a string (or str) to uppercase and lowercase
// * Try searching for: to_uppercase, to_lowercase

fn main() {
    let text1 = "Hello Rust";
    let text2 = "Good Morning".to_owned();

    println!("\nUsing borrowed (&str):\nTo lowercase: {:?}", text1.to_lowercase());
    println!("To uppercase: {:}", text2.to_uppercase());

    println!("\nUsing owned String:\nTo lowercase: {:?}", text2.to_lowercase());
    println!("To uppercase: {:}", text2.to_uppercase());
}