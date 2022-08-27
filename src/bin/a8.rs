//Topic: Organizing similar data using structs

// * Use an enum to create different flavors of drinks
enum Flavor {
    Sparkling,
    Sweet,
    Fluity,
}

// * Use a struct to store drink flavor and fluid ounce informations
struct Drink {
    flavor: Flavor,
    fluid_ounce: f64,                                                    
}

// * Use a function to print out the drink flavor and ounces
fn print_drink_info(drink: Drink) {
    // * Use a match expression to print the drink flavor
    match drink.flavor {
        Flavor::Sparkling => println!("Flavour: sparkling"),
        Flavor::Sweet => println!("Flavor: sweet"),
        Flavor::Fluity => println!("Flavor: fruity"),
    }

    println!("Oz: {:?}", drink.fluid_ounce);
}

fn main() {

    let sweet = Drink {
        flavor: Flavor::Sweet,
        fluid_ounce: 6.0,
    };
    print_drink_info(sweet);

    let fluity = Drink {
        flavor: Flavor::Fluity,
        fluid_ounce: 10.0,
    };
    println!();
    print_drink_info(fluity);


    let spark = Drink {
        flavor: Flavor::Sparkling,
        fluid_ounce: 8.0,
    };
    println!();
    print_drink_info(spark);

}