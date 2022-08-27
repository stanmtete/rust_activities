// Topic: Iterator
//
// Requirements
// * Triple the value of each item in a vector
// * Filter the data to only include values > 10
// * Print out each element using a for loop 
// 
// Notes:
// * Use an iterator chain to accomplish

fn main() {
    let scores = vec![5, 12, 14, 20, 1, 3, 10];

    let new_scores: Vec<_> = scores.iter().map(|num| num * 3).filter(|num| num > &10).collect();

    let mut i = 0;
    
    for score in new_scores {
        i = i + 1;
        println!("Score {:?}: {:?}", i,score);
    }
}