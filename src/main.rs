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
    println!("Hello, world!");
}
