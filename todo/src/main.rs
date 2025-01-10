use std::io::{self, Write};

fn main() {
    let mut todo_list: Vec<String> = vec![];

    loop {
        println!("\n=== Todo List Manager ===");
        println!("1. Add todo");
        println!("2. List todos");
        println!("3. Remove todo");
        println!("4. Exit");
        print!("\nChoose an option (1-4): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                print!("Enter your todo: ");
                io::stdout().flush().unwrap();
                let mut todo_item = String::new();
                io::stdin().read_line(&mut todo_item).unwrap();
                todo_list.push(todo_item.trim().to_string());
                println!("Todo added successfully!");
            }
            "2" => {
                if todo_list.is_empty() {
                    println!("No todos yet!");
                } else {
                    println!("\nYour todos:");
                    for (index, item) in todo_list.iter().enumerate() {
                        println!("{}. {}", index + 1, item);
                    }
                }
            }
            "3" => {
                if todo_list.is_empty() {
                    println!("No todos to remove!");
                } else {
                    println!("\nCurrent todos:");
                    for (index, item) in todo_list.iter().enumerate() {
                        println!("{}. {}", index + 1, item);
                    }
                    print!("Enter the number of the todo to remove: ");
                    io::stdout().flush().unwrap();
                    let mut remove_index = String::new();
                    io::stdin().read_line(&mut remove_index).unwrap();
                    
                    if let Ok(index) = remove_index.trim().parse::<usize>() {
                        if index > 0 && index <= todo_list.len() {
                            todo_list.remove(index - 1);
                            println!("Todo removed successfully!");
                        } else {
                            println!("Invalid todo number!");
                        }
                    } else {
                        println!("Please enter a valid number!");
                    }
                }
            }
            "4" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option! Please choose 1-4"),
        }
    }
}