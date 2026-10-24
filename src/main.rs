use std::collections::HashMap;
use std::fs::File; //Para crear  y abrir archivos
use std::io::Write; // Un trait para escribir bytes en un archivo
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Todo List")]
#[command(version = "1.0")]
#[command(about = "A simple CLI task manager built in Rust")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task to the list
    Add {
        item: String
    },
    /// Mark a task as complete
    Complete {
        item: String
    },
    /// Remove a task from the list
    Remove {
        item: String
    },
    /// List all current tasks
    List,
}
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

    fn list(&self) {
        if self.map.is_empty() {
            println!("You to-do list is empty.");
            return;
        }

        println!("Your tasks:");
        for (key, value) in &self.map {
            let status = if *value { "[ ]" } else { "[x]"};
            println!("{status} {key}");
        }
    }

    fn remove(&mut self, key: &String) -> Option<()> {
        match self.map.remove(key) {
            Some(_) => Some(()),
            None => None,
        }
    }
}

fn main() {
    let cli = Cli::parse();

    let mut todo = Todo::new().expect("Initialization of db failed");

    match &cli.command {
        Commands::Add { item } => {
            todo.insert(item.to_string());
            if let Err(why) = todo.save() {
                println!("An error occurred: {why}");
            } else {
                println!("Task added successfully.");
            }
        }
        Commands::Complete { item} => {
            if todo.complete(item).is_none() {
                println!("'{item}' is not present in the list");
            } else if let Err(why) = todo.save() {
                println!("An error occurred: {why}");
            } else {
                println!("Task marked as complete");
            }
        }
        Commands::Remove {item} => {
            if todo.remove(item).is_none() {
                println!("´{item}' is not present in the list");
            } else if let Err(why) = todo.save() {
                println!("An error ocurred: {why}");
            } else {
                println!("Task removed successfuly");
            }
        }
        Commands::List => {
            todo.list();
        }
    }
}