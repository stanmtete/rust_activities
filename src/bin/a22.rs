// Topic: Testing
// 
// Requirements
// * Write for the existing program to ensure proper functionality
// 
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program
// * There are intentional bugs in the program that need to be fixed.
// * Check the documentation comments for the function to determine how they should operate.

/// Ensure n is >= lower and <= upper
pub fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    }else if n > upper {
        upper
    }else {
        n
    }
}

/// Divide a and b
pub fn div(a: i32, b: i32) -> Option<i32> {
   if b == 0 {
     None
   }else {
    Some(a / b)
   }
}


/// Take two string and place them immediately on the console
pub fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {}

#[cfg(test)]

mod test {
    use crate::*;

    #[test]
    fn clapper_upper() {
        let result = clamp(10, 100, 100);
        let expected = 100;

        assert_eq!(result, expected, "Should be 100");
    }

    #[test]
    fn check_div() {
        let result = div(1, 1);
        let expected = Some(1);

        assert_eq!(result, expected, "Should be 1")
    }

    #[test]
    fn check_div_by_zero() {
        let result = div(1, 0);
        let expected = None;

        assert_eq!(result, expected, "Division by zero error"); 
    }
    #[test]
    fn check_contact() {
        let result = concat("a", "b");
        let expected = "ab".to_owned();

        assert_eq!(result, expected, "Should be place immediately together")
    }
}

