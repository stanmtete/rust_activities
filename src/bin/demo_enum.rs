//Demo: Enumeration

//Creating the enum with possible data
#[allow(dead_code)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}


fn main() {
    
    let go = Direction::Left;

    match go {
        Direction::Left => println!("Go left"),
        Direction::Right => println!("Go right"),
        Direction::Up => println!("Go up"),
        Direction::Down => println!("Go down"),
    }
}