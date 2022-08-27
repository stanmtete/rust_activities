// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use and Option<i32>



// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use and Option<i32>
struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let students = Student {
        name: String::from("Moses"),
        locker: Some(9361),
    };

    println!("Student name: {:?}", students.name);
    match students.locker {
        Some(num)=> println!("Locker number: {:?}", num),
        None => println!("No locker assigned!"),
    }

   
}