// Topic: Result and the question mark operator
//
// Requirements
// * Determine if an employee can access a building using a digital keycard
// * Employee that can access the building are: 
// * Maintenance crews 
// * Marketing department employee 
// * Managers

// * Other employee that work at the company are:- 
// * Line supervisors 
// * Kitchen staff
// * Assembly technicians
// * Ensure that the terminated employee cannot access the building 
// regardless of their position
//
// Notes: 
// * Use an enum to represent all type of employee
// * Use a struct to store the employee type and whether they are still employed 
// * Use a function that returns a Result to determine if the employee may enter the building 
// * Print whether the employee may access the building 
// * Must use a function that utilizes the  question mark operator to do this

// * Use an enum to represent all type of employee
#[allow(dead_code)]
enum Position {
    Maintenance,
    Marketing,
    Managers,
    LineSupervisor,
    KitchenStaff,
    AssemblyTech
}

// * The enum of status
#[allow(dead_code)]
enum Status {
    Active,
    Terminated,
}

// * Use a struct to store the employee type and whether they are still employed 
struct Employee {
    position: Position,
    status: Status,
}

// * Use a function that returns a Result to determine if the employee may enter the building 
fn try_access(employee: &Employee) -> Result<(), String> {
    match employee.status {
        Status::Terminated => return Err("terminated".to_owned()),
        _ => (),
    }

    match employee.position {
        Position::Maintenance => Ok(()),
        Position::Marketing => Ok(()),
        Position::Managers => Ok(()),
        _ => Err("Invalid position".to_owned()),
    }
}

// * Print whether the employee may access the building 
fn print_acces(employee: &Employee) -> Result<(), String>{
    try_access(employee)?;
    println!("Access Ok");
    Ok(())
}

fn main() {
    let manager = Employee {
        position: Position::Managers,
        status: Status::Active,
    };

    match print_acces(&manager) {
        Err(e) => println!("Access denied {:?}", e),
        _ => (),
    }
}