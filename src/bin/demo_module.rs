// The module are important because helps to organize you're code and also self contain and easy to follow.

mod greet {
    pub(crate) fn hello() {
        println!("Hello");
    }

    pub(crate) fn goodbye() {
        println!("Goodbye");
    }
}

mod math {
    pub(crate) fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub(crate) fn sub(a: i32, b: i32) -> i32 {
        a - b
    }
}


fn main() {
    // To use you're module for a specific function just simply do
    // 1st: Way
    use greet::hello;
    hello();

    greet::goodbye();
    greet::hello();

    // Other way to import and use all the functions in a certain module.
    use greet::*;
    goodbye();
    hello();

    use math::*;
    println!("1 + 3 = {:?}", add(1, 3));
    println!("3 - 1 = {:?}", sub(3, 1));
}