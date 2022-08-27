// In these demo show how to create the String owned and borrowed

struct LineItem {
    name: String,
    count: i32,
}

fn print_name(name: &str) {
    println!("Name: {:?}", name);
}

fn main() {
    let receipt = vec![
        LineItem {
            name: "Cereal".to_owned(),
            count: 3,
        },
        LineItem {
            name: String::from("Fruit"),
            count: 2,
        },
    ];

    for item in receipt {
        print_name(&item.name);
        println!("Count: {:?}", item.count);
    }
}