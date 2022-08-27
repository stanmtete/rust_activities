// Testing in rust, testing is important part in software development which help you to check 
// for error or successfully of you're code.

pub fn all_caps(word: &str) -> String {
    word.to_uppercase()
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn check_all_caps() {
        let result = all_caps("hello");
        let expected = String::from("hello");
        
        assert_eq!(result, expected, "String should be all in uppercase");
    }
}
