use std::io::{Write, stdin, stdout};
#[derive(Debug, PartialEq)]
struct Task {
    task: String,
    is_completed: bool,
}
fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    loop {
        main_menu();
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        let user_input = input.trim().parse();
        match user_input {
            Ok(1) => view_todos(&tasks),
            Ok(2) => add_todo(&mut tasks),
            Ok(3) => edit_todo(&mut tasks),
            Ok(4) => break,
            _ => println!("Invalid Output"),
        };
    }
}
fn main_menu() {
    println!("\n");
    println!("------- RUST Todo Applications ------- ");
    println!("Press any keys between 1 to 4 to perform action: ");
    println!("1. View All Todos");
    println!("2. Add a Todo");
    println!("3. Edit a todo");
    println!("4. Exit");
    println!("\n");
    println!("> ");
    stdout().flush().expect("Failed to flush the line");
}

fn view_todos(todos: &Vec<Task>) {
    if !todos.is_empty() {
        for (index, task) in todos.iter().enumerate() {
            println!(
                "Task {index}: {} {}",
                if task.is_completed { "✅" } else { " " },
                task.task
            );
        }
    } else {
        println!("Wohoo, There is no pending task");
    }
}
fn add_todo(tasks: &mut Vec<Task>) {
    stdout().flush().expect("Failed to flush the line");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    println!(">");
    tasks.push(Task {
        task: input.trim().to_string(),
        is_completed: false,
    });
}
fn edit_todo(tasks: &mut Vec<Task>) {
    stdout().flush().expect("Failed to flush the line");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    println!(">");
    match input.trim().parse::<usize>() {
        Ok(num) => {
            if num > tasks.len() {
                println!("Task is absent in list, please try again!");
                return;
            }
            tasks[num].is_completed = !tasks[num].is_completed;
        }
        Err(error) => println!("{}", error),
    }
}
#[cfg(test)]
mod tests {
    use crate::{Task, add_todo, edit_todo, view_todos};

    #[test]
    fn test_view_todos() {
        let mut todos: Vec<Task> = Vec::new();
        view_todos(&todos);
    }
    #[test]
    fn test_add_todos() {
        let mut todos: Vec<Task> = Vec::new();
        add_todo(&mut todos);
        assert_eq!(
            todos[0],
            Task {
                task: "Reading Book".to_string(),
                is_completed: false
            }
        );
    }
    #[test]
    fn test_edit_todos() {
        let mut todos: Vec<Task> = Vec::new();
        add_todo(&mut todos);
        edit_todo(&mut todos);
        assert_eq!(
            todos[0],
            Task {
                task: "Reading Book".to_string(),
                is_completed: true
            }
        )
    }
}
