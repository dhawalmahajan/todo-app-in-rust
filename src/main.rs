use std::io::stdin;

fn main() {
    println!("------- RUST Todo Applications ------- ");
    println!("Press any keys between 1 to 4 to perform action: ");
    println!("1. View All Todos");
    println!("2. Add a Todo");
    println!("3. Edit a todo");
    println!("4. Exit");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    let mut user_input = input.trim().parse();
    match user_input {
        Ok(1) => println!(" View All Todos"),
        Ok(2) => println!("Add a Todo"),
        Ok(3) => println!("  Edit a todo"),
        Ok(4) => println!(" Exit"),
        _ => println!("Invalid Output"),
    };
}
