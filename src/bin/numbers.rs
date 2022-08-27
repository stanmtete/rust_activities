

fn main() {

    let mut num= 1;

    while num <=  100 {
       
        match num {
            1 => println!("First pass"),
            2 => println!("Second pass"),
            3 => println!("Third pass"),
            _ => println!("Failed"),
        }
        num = num + 1;
    }
}