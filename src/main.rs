use std::collections::HashMap;
use std::fs::File; //Para crear  y abrir archivos
use std::io::Write; // Un trait para escribir bytes en un archivo

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

    fn save(&self) -> Result<(), std::io::Error> {
        let mut file = File::create("db.txt")?; //Crear archivo

        for (key, value) in &self.map {
            let record = format!("{}\t{}\n", key, value);

            file.write_all(record.as_bytes())?;
        }
        Ok(())
    }
}

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action");
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
