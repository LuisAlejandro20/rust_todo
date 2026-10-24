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

    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }
}

fn main() {
    let action = std::env::args().nth(1).expect("Please specify and action");
    let item = std::env::args().nth(2).expect("Please specify an item");

    let mut todo = Todo::new().expect("Initialization of db failed");

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(why) => println!("An error ocurred: {why}"),
        }
    }
}
