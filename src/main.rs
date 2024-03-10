use std::io::{self, Write};
fn main() {
    let mut todos: Vec<String> = Vec::new();

    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let tokens: Vec<&str> = input.trim().split_whitespace().collect();

        match tokens[..] {
            ["add", item] => {
                let item = String::from(item);
                println!("Added item: {item}");
                todos.push(item);
            }
            ["remove", item] => {
                let item = String::from(item);
                if let Some(index) = todos.iter().position(|x| *x == item) {
                    todos.remove(index);
                }
                println!("Removed item: {item}");
            }
            ["show"] => println!("{:?}", todos),
            ["exit"] => {
                println!("Exited applciation");
                break;
            }
            _ => println!("invalid input"),
        }
    }
}
