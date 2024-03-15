use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

#[derive(Serialize, Deserialize)]
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
    create_save_path();
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

            ["save", name] => {
                let data = create_json_data(&todos);

                let file_path = format!("./saves/{}.json", name);
                let path = Path::new(&file_path);
                let mut file = File::create(path).expect("Unable to create file");
                file.write_all(data.as_bytes())
                    .expect("Unable to write data");

                println!("Saved data to file: {}", name);
            }

            ["load", name] => {
                let file_path = format!("./saves/{}.json", name);
                let data = read_json_data(file_path);
                match data {
                    Ok(items) => {
                        todos = items;
                        println!("loaded from file {name}")
                    }
                    Err(e) => eprintln!("{}", e),
                }
            }

            ["help"] => {
                println!("Available commands:");
                println!("  add <item_name>   - Add a new item to the list");
                println!("  remove <item_name>- Remove an item from the list");
                println!("  toggle <item_name>- Toggle the status of an item (done/undone)");
                println!("  show              - Show all items in the list");
                println!("  save <save_name>  - Save current todos to file");
                println!("  load <save_name>  - Load todos from file");
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

fn create_save_path() {
    let save_path = format!("./saves");
    let path = Path::new(&save_path);
    std::fs::create_dir_all(path).expect("Unable to create directory");
}

fn create_json_data(todos: &Vec<Item>) -> String {
    serde_json::to_string(&todos).expect("Cant serialize")
}

fn read_json_data(path: String) -> std::io::Result<Vec<Item>> {
    let data = fs::read_to_string(path)?;
    let items: Vec<Item> = serde_json::from_str(&data)?;
    Ok(items)
}
