use std::collections::HashMap;
use std::fs::File; //Para crear  y abrir archivos
use std::io::Write; // Un trait para escribir bytes en un archivo

struct Todo {
    map: HashMap<String, bool>
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        let mut map = HashMap::new();

        let contents = std::fs::read_to_string("db.txt");
        
        if let Ok(data) = contents {
            for line in data.lines() {
                let mut parts = line.split('\t');

                let key = parts.next().expect("Missing task description");
                let val = parts.next().expect("Missing task status");

                let is_active = val.parse::<bool>().unwrap_or(true);

                map.insert(key.to_string(), is_active);
            }
        }

        Ok(Todo { map })
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

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => {
                *v = false;
                Some(())
            }
            None => None,
        }
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
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{item}' is not present in the list"),
            Some(_) => match todo.save() {
                Ok(_) => println!("todo saved"),
                Err(why) => println!("An error occurred: {why}"),
            },
        }
    } else {
        println!("Unknown action. Use 'add' or 'complete'.");
    }
}
