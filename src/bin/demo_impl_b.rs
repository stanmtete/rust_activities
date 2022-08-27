//The improve demo_impl to use the impl keyword to organize the struct code

struct Temperature {
    degrees_f: f64,
}

impl Temperature {

    //Creating the new freezing temperature 
    fn freezing() -> Self {
        Self {
            degrees_f: 32.0
        }
    }
    
    //Creating the new freezing temperature 
    fn boiling() -> Self {
        Self {
            degrees_f: 212.0
        }
    }
    //The function to print out the temperature 
    fn show_temp(&self) {
        println!("{:?} degree F", self.degrees_f);
    }
}

fn main() {
    //Hotness 
    let hot = Temperature {
        degrees_f: 99.9
    };

    hot.show_temp();

    //Freezing
    let cold = Temperature::freezing();
    cold.show_temp();

    //Boiling 
    let boil = Temperature::boiling();
    boil.show_temp();
}