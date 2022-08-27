// In this program we are going to work with enum, struct and function 

/// Enum generates bachelor courses 
#[allow(dead_code)]
enum Course {
    Banking,
    Engineering,
    Insurance,
}

/// Implements the course by using the print function as well as the match expression to match to a specific choice
impl  Course {
    fn print(&self) {
        match self {
            Course::Banking => println!("Course: Banking"),
            Course::Engineering => println!("Course: Engineering"),
            Course::Insurance => println!("Course: Insurance"),
        }
    }
}

/// The struct of type Student with basic informations
struct Student {
    name: String,
    id: i64,
    major: Course,
    gpa: f64,
}

/// The implementations of the struct to print out students details
impl Student {
    fn print_info(&self) {
        println!("Name: {:?}", self.name);
        println!("ID: {:?}", self.id);
        self.major.print();
        println!("GPA: {:?}", self.gpa);
    }
}
fn main() {
    
    // Struct object with associated members
    let s1 = Student {
        name: "Brown Juca".to_owned(),
        id: 200110022,
        major: Course::Engineering,
        gpa: 4.4,
    };

    s1.print_info();
}