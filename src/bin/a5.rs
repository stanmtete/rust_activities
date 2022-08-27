fn main() {

    let mut num = 1;

    loop {
        println!("{:?}", num);

        if num == 4 {
            break;
        }

        num = num + 1;

    }

    println!("done!");
}