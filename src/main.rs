use std::collections::HashMap;

struct Todo {
    map: HashMap<String, bool>
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        Ok(Todo {
            map: HashMap::new(),
        })
    }
}

fn main() {
    println!("Hello, world!");
}
