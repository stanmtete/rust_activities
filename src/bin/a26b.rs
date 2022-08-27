/*
Topic: Inline module

Summary
---------
The existing program is complete, but all the code exists
in a single module. This code can benefit from being organized
into multiple modules

Requirements
* Organize the code into two modules based on their functionality:
- msg: String formatting functions
- math: math functions
* Update the main function to use the functionality from the modules

Notes:
* After moving the functions into the modules, try running.
`cargo check --bin a26b` to get a listing of required code changes
*/


// First: Module
mod msg {
    pub(crate) fn to_uppercase_text(str: &str) -> String {
        return str.to_uppercase();
    }

    pub(crate) fn to_lower_case_msg(msg: &str) -> String {
        return msg.to_lowercase();
    }
}

// Second: Module
mod math {

    pub(crate) fn sum(text1: i32, text2: i32) -> i32 {
        text1 + text2
    }

    pub(crate) fn avg(total: i32) -> i32 {
        total / 2
    }
}

fn main() {
    // Calling the sum function from math module and assign the returned value to the variabele 
    let total = math::sum(3,7);
    let my_avg = math::avg(total);

    use msg::*;
    let upper_str = to_uppercase_text("hello");
    let lower_str = to_lower_case_msg("THIS IS the BEst coding evER");

    println!("Total test 1 & 2: {:?}\nAverage is: {:?}", total, my_avg);
    println!("To upper string: {:?}\nTo lower string: {:?}", upper_str, lower_str);
    
}