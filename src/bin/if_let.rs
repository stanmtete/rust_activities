// if let when working with some data.
// It's the shorter of the match expression 

#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    Brown,
}

fn main() {
    let maybe_user = Some("Jerry");

    match maybe_user {
        Some(user) => println!("Match user = {:?}", user),
        None => println!("Match user: Not found")
    }

    if let Some(user) = maybe_user {
        println!("If let user = {:?}", user);
    } else {
        println!("If let: No user");
    }

    let red = Color::Red;

    if let Color::Red = red {
        println!("It's red");
    }else {
        println!("It's not red");
    }
}