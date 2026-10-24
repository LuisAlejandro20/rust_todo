use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct Task {
    active: bool,
    priority: u8
}
#[derive(Serialize, Deserialize)]
pub struct Todo {
    map: HashMap<String, Task>
}

impl Todo {
    pub fn new() -> Result<Todo, std::io::Error> {
        let file = std::fs::File::open("db.json");
        match file {
            Ok(f) => {
                let map = serde_json::from_reader(f)?;
                Ok(Todo {map})
            }
            Err(_) => Ok(Todo {
                map: HashMap::new()
            })
        }
    }

    pub fn insert(&mut self, key: String, priority: u8) {
        let new_task = Task {
            active: true,
            priority,
        };
        self.map.insert(key, new_task);
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let file = std::fs::File::create("db.json")?;
        serde_json::to_writer_pretty(file, &self.map)?;
        Ok(())
    }

    pub fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(task) => {
                task.active = false;
                Some(())
            }
            None => None,
        }
    }

    pub fn remove(&mut self, key: &String) -> Option<()> {
        match self.map.remove(key) {
            Some(_) => Some(()),
            None => None,
        }
    }

    pub fn list(&self) {
        if self.map.is_empty() {
            println!("Your to-do list is empty.");
            return;
        }

        let mut tasks: Vec<(&String, &Task)> = self.map.iter().collect();

        tasks.sort_by(|a,b| b.1.priority.cmp(&a.1.priority));

        println!("Your tasks:");
        for (key, task) in tasks {
            let status = if task.active { "[ ]" } else { "[ ]" };
            println!("{} {} (Priority: {})", status, key, task.priority);
        }
    }
}