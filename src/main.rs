use core::task;
use std::io::stdin;
#[derive(Debug)]
struct Task {
    task: String,
    is_Completed: bool,
}
fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    main_menu();
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    let mut user_input = input.trim().parse();
    match user_input {
        Ok(1) => view_todos(&tasks),
        Ok(2) => println!("Add a Todo"),
        Ok(3) => println!("  Edit a todo"),
        Ok(4) => println!(" Exit"),
        _ => println!("Invalid Output"),
    };
}
fn main_menu() {
    println!("------- RUST Todo Applications ------- ");
    println!("Press any keys between 1 to 4 to perform action: ");
    println!("1. View All Todos");
    println!("2. Add a Todo");
    println!("3. Edit a todo");
    println!("4. Exit");
}

fn view_todos(todos: &Vec<Task>) {
    if todos.len() > 0 {
        for (index, task) in todos.iter().enumerate() {
            println!(
                "Task {index}: {} {}",
                if task.is_Completed { "✅" } else { " " },
                task.task
            );
        }
    } else {
        println!("Wohoo, There is no pending task");
    }
}
