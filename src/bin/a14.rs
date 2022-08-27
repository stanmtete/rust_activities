// Topic: String
//
// Requirements
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes
// * Use a struct for person age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

// * Use a struct for person age, name, and favorite color
struct Person {
    name: String,
    fav_color: String,
    age: i32,
}

// * The name and colors should be printed using a function
fn print_info(name: &str, color: &str) {
    println!("Name: {:?}", name);
    println!("Favorite Color: {:?}", color);
}

fn main() {
      // * Create and store at least 3 people in a vector
     let people = vec![
        Person {
            name: "John Doe".to_owned(),
            fav_color: "Black".to_owned(),
            age: 10,
        },
        Person {
            name: "Moses Kisoki".to_owned(),
            fav_color: "Brown".to_owned(),
            age: 8,
        },
        Person {
            name: "Orlan Kiweru".to_owned(),
            fav_color: "Yellow".to_owned(),
            age: 20,
        },
     ];

     // * Iterate through the vector using a for..in loop
    for person in people {

        // * Use an if expression to determine which person's info should be printed
        if person.age <= 10 {
            print_info(&person.name, &person.fav_color);
            println!();
        }
    }
}