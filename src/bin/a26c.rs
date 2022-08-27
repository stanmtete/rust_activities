/*
Topic: Inline module

Summary
--------
The existing program is complete, but all the code exists
in a single module. This code can bene&fit from being organized
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

mod msg {
    pub fn trim(msg: &str) -> &str {
        msg.trim()
    }
    
    // pub fn capitalize(msg: &str) -> std::borrow::Cow<'_, str> {
    //     if let Some(letter) = msg.get(0..1) {
    //         format!("{}{}", letter.to_uppercase(), &msg)
    //     }else {
    //         msg.into()
    //     }
    // }
    
    pub fn exciting(msg: &str) -> String {
        format!("{}!", msg)
    }
}

mod math {
    pub fn add(lhl: isize, rhs: isize) -> isize {
        lhl + rhs
    }
    
    
    pub fn sub(lhl: isize, rhs: isize) -> isize {
        lhl - rhs
    }
    
    
    pub fn mul(lhl: isize, rhs: isize) -> isize {
        lhl *   rhs
    }    
}

fn main() {
    // Part 1: math functions
    let result = {
        let two_plus_two = math::add(2,2);
        let three = math::sub(two_plus_two, 1);
        math::mul(three, three);
    };

    // Ensure we have a correct result
    // assert_eq!(result, 9);
    println!("(2 + 2 - 1) * 3 = {:?}", result);

   {
    use msg::{exciting, trim};

    let hello = {
        let msg = "hello ";
        trim(msg);
        // capitalize(msg);
    };

    let world = {
        let msg = "world";
        exciting(msg);
    };

    let msg = format!("{:?}, {:?}", hello, world);

    assert_eq!(&msg, "Hello, world!");
    println!("{}", msg);
    
   }

}



