//Creating the expressions demo to check if the user has access to the files


//Create an enum of access users
#[allow(dead_code)]
enum Access {
    Admin,
    Manager,
    User,
    Guest,
}

fn main() {
    //Variable to hold access level 
    let access_level = Access::Guest;

    let can_access_file = match access_level {
        Access::Admin => true,
        _ => false,
    };

    println!("Can access file: {:?}", can_access_file);
}