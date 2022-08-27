// The derive macro applies to the enum and struct and hence make easily to work with them
// Also derive can be used to Clone and Copy the piece of data from one func to another 
// Which means that the ownership is not transfered

#[allow(dead_code)]
#[derive(Debug)]
enum Position {
    Manager,
    Supervisor,
    Worker,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Employee {
    position: Position,
    work_hours: i64,
}

fn main() {
    let me = Employee {
        position: Position::Manager,
        work_hours: 40,
    };

    println!("{:?}", me.position);
}