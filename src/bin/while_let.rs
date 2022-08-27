// It's useful when working with iterators, which makes easy to check if data not found in some iterator

fn main() {
    let mut data = Some(3);

    while let Some(i) = data {
        println!("Loop: {:?}", i);
        data = None;
    }

    let numbers = vec![1, 2, 3];
    let mut number_iter = numbers.iter();

    while let Some(num) = number_iter.next() {
        println!("Num = {:?}", num);
    }

    println!("done");
}