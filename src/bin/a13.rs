// Topic: Vectors
//
// Requirements
// * Print 10, 20, "thirty" and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() to print the number of elements in a vector

struct  Number {
    my_num: i32,
}

fn main() {
    // * Use a vector to store 4 numbers
    let my_numbers = vec![
        Number { my_num: 10 },
        Number { my_num: 20 },
        Number { my_num: 30 },
        Number { my_num: 40 },
    ];


    for num in &my_numbers {
        // * Determine whether to print the number or print "thirty" inside the loop
        match num.my_num {
            30 => println!("thirty"),
            _ => println!("Number: {:?}", num.my_num),
        }
    }

// * Use the .len() to print the number of elements in a vector
println!("Number of elements: {:?}", my_numbers.len());


}