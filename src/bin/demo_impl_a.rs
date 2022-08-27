//In this demo shows how to organize the struct to the implementation (impl) keyword
// and makes the code easly to read and follow.

struct Temperature {
    degrees_f: f64,
}

fn show_temp(temp: Temperature) {
    println!("{:?} degree F", temp.degrees_f);
}

fn main() {
    let hot = Temperature {
        degrees_f: 99.9
    };

    show_temp(hot);
}