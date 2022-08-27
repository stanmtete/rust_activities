#[allow(dead_code)]
enum Light {
    Bright,
    Dull,
}

fn display_light(light: Light) {
    match light {
        Light::Bright => println!("bright"),
        Light::Dull => println!("dull"),
    }
}


fn main() {
    let dull = Light::Dull;

    display_light(dull);
    // display_light(dull);
}