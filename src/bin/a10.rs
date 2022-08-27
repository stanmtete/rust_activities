//Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100

//Notes::
// * Use a boolean variable set to an expression
// that determines whether the value is > 100 or < 100
// * Use a function to print the message
// * Use a match expression to determine which message to print

// * Use a function to print the message
fn print_message(num: bool) {
    // * Use a match expression to determine which message to print
    match num {
        true => println!("its big"),
        false => println!("its small"),
    }
}
 fn main() {
    let num = 30;
    // * Use a boolean variable set to an expression
    // that determines whether the value is > 100 or < 100
    let is_gt_100 = num > 100;

    print_message(is_gt_100);
}