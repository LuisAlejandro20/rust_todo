use clap::{Parser, Subcommand};

mod todo;

use todo::Todo;


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
        item: String,
        #[arg(short, long, default_value_t = 1)]
        priority: u8,
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


fn main() {
    let cli = Cli::parse();

    let mut todo = Todo::new().expect("Initialization of db failed");

    match &cli.command {
        Commands::Add { item, priority } => {
            todo.insert(item.to_string(), *priority);
            if let Err(why) = todo.save() {
                println!("An error occurred: {why}");
            } else {
                println!("Task added successfully with priority {priority}.");
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