use std::collections::HashMap;
use std::fs::File; //Para crear  y abrir archivos
use std::io::Write; // Un trait para escribir bytes en un archivo
use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};

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
#[derive(Serialize, Deserialize)]
struct Todo {
    map: HashMap<String, bool>
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        let file = std::fs::File::open("db.json");

        match file {
            Ok(f) => {
                let map = serde_json::from_reader(f)?;
                Ok(Todo {map})
            }
            Err(_) => {
                Ok( Todo {
                    map: HashMap::new()
                })
            }
        }
    }

    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }

    fn save(&self) -> Result<(), std::io::Error> {
        let file = std::fs::File::create("db.json")?;
        serde_json::to_writer_pretty(file, &self.map)?;
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