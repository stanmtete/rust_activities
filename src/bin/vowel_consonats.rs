// In this simple program is to print out vowel and consonants using for..in and match expression

fn main() {

    for alph in 'a'..='z' {

        let sorted = match alph {
            'a' => "vowel",
            'e' => "vowel",
            'i' => "vowel",
            'o' => "vowel",
            'u' => "vowel",
            _ => "consonant",
        };
        println!("{:?} = {:?}", alph, sorted);
    }
}