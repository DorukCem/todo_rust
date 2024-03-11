use std::fmt;
use std::io::{self, Write};

struct Item {
    name: String,
    done: bool,
}
impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} [{}]", self.name, if self.done { 'X' } else { ' ' })
    }
}

fn main() {
    let mut todos: Vec<Item> = Vec::new();

    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let tokens: Vec<&str> = input.split_whitespace().collect();

        match tokens[..] {
            ["add", item_name] => {
                let item = Item {
                    name: String::from(item_name),
                    done: false,
                };
                todos.push(item);
                println!("Added item: {item_name}");
            }

            ["remove", item_name] => {
                if let Some(index) = todos.iter().position(|x| x.name == item_name) {
                    todos.remove(index);
                    println!("Removed item: {item_name}");
                } else {
                    eprintln!("Could not find item: {item_name}");
                }
            }

            ["toggle", item_name] => {
                if let Some(index) = todos.iter().position(|x| x.name == item_name) {
                    todos[index].done = !todos[index].done;
                    println!("Toggled item: {item_name}");
                } else {
                    eprintln!("Could not find item: {item_name}");
                }
            }

            ["show"] => {
                for item in &todos {
                    println!("   {item}");
                }
            }

            ["help"] => {
                println!("Available commands:");
                println!("  add <item_name>   - Add a new item to the list");
                println!("  remove <item_name>- Remove an item from the list");
                println!("  toggle <item_name>- Toggle the status of an item (done/undone)");
                println!("  show              - Show all items in the list");
                println!("  help              - Show this help message");
                println!("  exit              - Exit the application");
            }

            ["exit"] => {
                println!("Exited applciation");
                break;
            }
            _ => eprintln!("invalid input"),
        }
    }
}
