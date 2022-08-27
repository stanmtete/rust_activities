// Hashmap are usefull when you know the keys to match for a certain value

// First: import he standard library to use HashMap collection
use std::collections::HashMap;

#[derive(Debug)]
struct Contents {
    content: String
}

fn main() {
    let mut lockers = HashMap::new();

    lockers.insert(1, Contents{content: "Stuff".to_owned()});
    lockers.insert(2, Contents { content: "T-shirt".to_owned() });
    lockers.insert(3, Contents { content: "Gym_shorts".to_owned() });


    //Iterate through the lockers 
    for (locker_num, content) in lockers.iter() {
        println!("Number: {:?} Content: {:?}", locker_num, content.content);
    }
}