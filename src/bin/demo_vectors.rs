// In this demo shows how to create the vector using the macro 
// and use the for in loop to iterate through the vector items

struct Test {
    score: i32,
}

fn main() {
    let my_scores = vec![
        Test { score: 90 },
        Test { score: 40 },
        Test { score: 80 },
        Test { score: 50 },
    ];

    //Iterate through the vector items
    for test in my_scores {
        println!("Score: {:?}", test.score);
    }
}