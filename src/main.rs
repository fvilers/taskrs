use std::path::PathBuf;

use clap::{Parser, Subcommand};
use home::home_dir;
use taskrs::prelude::*;

#[derive(Parser)]
#[command(about = "A simple command line to-do manager")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(
        short,
        long,
        help = "The path where to find and store the tasks.json file"
    )]
    path: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Add a task")]
    Add { task: String },

    #[command(about = "List tasks")]
    List {
        #[arg(short, long, help = "Include done tasks")]
        all: bool,
    },

    #[command(about = "Update a task")]
    Update { id: u32, task: String },

    #[command(about = "Mark a task as done")]
    Done { id: u32 },

    #[command(about = "Mark a task as undone")]
    Undone { id: u32 },

    #[command(about = "Delete a task")]
    Delete { id: u32 },

    #[command(about = "Swap tasks")]
    Swap { id1: u32, id2: u32 },

    #[command(about = "Empty the task list")]
    Reset {
        #[arg(short, long, help = "Don't prompt for confirmation")]
        force: bool,
    },

    #[command(about = "Get information about your tasks")]
    Infos,
}

fn main() {
    let cli = Cli::parse();
    let file_path = cli
        .path
        .unwrap_or(home_dir().expect("Could not determine user's home directory"))
        .join("tasks.json");

    match cli.command {
        Some(Commands::Add { task }) => add_task(task, file_path),
        Some(Commands::List { all }) => list_tasks(all, file_path),
        Some(Commands::Update { id, task }) => update_task(id, task, file_path),
        Some(Commands::Done { id }) => mark_task(id, true, file_path),
        Some(Commands::Undone { id }) => mark_task(id, false, file_path),
        Some(Commands::Delete { id }) => delete_task(id, file_path),
        Some(Commands::Swap { id1, id2 }) => swap_tasks(id1, id2, file_path),
        Some(Commands::Reset { force }) => reset_tasks(force, file_path),
        Some(Commands::Infos) => infos(file_path),
        None => {}
    }
}
