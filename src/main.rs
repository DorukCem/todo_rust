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
        let tokens: Vec<&str> = input.trim().split_whitespace().collect();

        match tokens[..] {
            ["add", item_name] => {
                let item_name = String::from(item_name);
                println!("Added item: {item_name}");
                let item = Item {
                    name: item_name,
                    done: false,
                };
                todos.push(item);
            }

            ["remove", item_name] => {
                let item_name = String::from(item_name);
                if let Some(index) = todos.iter().position(|x| (*x).name == item_name) {
                    todos.remove(index);
                }
                println!("Removed item: {item_name}");
            }

            ["toggle", item_name] => {
                let item_name = String::from(item_name);
                if let Some(index) = todos.iter().position(|x| (*x).name == item_name) {
                    todos[index].done = !todos[index].done
                }
            }

            ["show"] => {
                for item in todos.iter() {
                    println!("   {item}");
                }
            }

            ["exit"] => {
                println!("Exited applciation");
                break;
            }
            _ => println!("invalid input"),
        }
    }
}
