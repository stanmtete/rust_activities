// Working with menus with several choices
// we will work with enumaration and get user inputs as string slice

#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("Menu choice not found".to_owned()),
    }
}

fn print_choice(choice: &MenuChoice) {
    println!("Choice: {:?}", choice);
}

fn pick_choice(input: &str) -> Result<(), String> {
    let choice: MenuChoice = get_choice(input) ?;
    print_choice(&choice);
    Ok(())
}

fn main() {
    let choice = pick_choice("start");
    println!("Choice value: {:?}", choice);
}