use clap::{Parser, Subcommand};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::io::{self, Write};

/// Path to the JSON file where tasks are stored
const FILE_PATH: &str = "tasks.json";

/// Represents a task
#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: Uuid,
    description: String,
    completed: bool,
}

/// Load tasks from JSON file
fn load_tasks() -> Vec<Task> {
    if Path::new(FILE_PATH).exists() {
        let data = fs::read_to_string(FILE_PATH).expect("Failed to read file");
        serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}

/// Save tasks to JSON file
fn save_tasks(tasks: &Vec<Task>) {
    let data = serde_json::to_string_pretty(tasks).expect("Failed to serialize");
    fs::write(FILE_PATH, data).expect("Failed to write file");
}

/// CLI Task Manager
#[derive(Parser)]
#[command(name = "task")]
#[command(about = "A simple CLI Task Manager in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { description: String },
    List,
    Done { id: Uuid },
    Delete { id: Uuid },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { description } => {
            let mut tasks = load_tasks();
            let task = Task {
                id: Uuid::new_v4(),
                description: description.clone(),
                completed: false,
            };
            tasks.push(task);
            save_tasks(&tasks);
            println!("✅ Task added: {}", description);
        }
        Commands::List => {
            let tasks = load_tasks();
            if tasks.is_empty() {
                println!("📭 No tasks found.");
            } else {
                for task in &tasks {
                    let status = if task.completed { "✅ Done" } else { "⏳ Pending" };
                    println!("[{}] {} - {}", task.id, status, task.description);
                }
            }
        }
        Commands::Done { id } => {
            let mut tasks = load_tasks();
            if let Some(task) = tasks.iter_mut().find(|t| t.id == *id) {
                task.completed = true;
                save_tasks(&tasks);
                println!("✅ Task '{}' marked as done!", id);
            } else {
                println!("❌ Task not found.");
            }
        }
        Commands::Delete { id } => {
            let mut tasks = load_tasks();
            let initial_len = tasks.len();
            tasks.retain(|task| task.id != *id);
            
            if tasks.len() < initial_len {
                save_tasks(&tasks);
                println!("🗑️ Task '{}' deleted!", id);
            } else {
                println!("❌ Task not found.");
            }
        }
    }
}
