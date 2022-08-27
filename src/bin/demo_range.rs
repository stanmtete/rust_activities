// Topic: Range in rust

fn main() {
    let _range = 1..=3;
    let range = 1..=4;

    for num in range {
        println!("Num: {:?}", num);
    }
}