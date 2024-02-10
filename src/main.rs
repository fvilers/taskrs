use std::path::PathBuf;

use clap::{Parser, Subcommand};
use home::home_dir;
use taskrs::TaskStore;

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
        .unwrap_or_else(|| home_dir().expect("Could not determine user's home directory"))
        .join("tasks.json");
    let store = TaskStore::new(file_path);

    match cli.command {
        Some(Commands::Add { task }) => store.add_task(task),
        Some(Commands::List { all }) => store.list_tasks(all),
        Some(Commands::Update { id, task }) => store.update_task(id, task),
        Some(Commands::Done { id }) => store.mark_task(id, true),
        Some(Commands::Undone { id }) => store.mark_task(id, false),
        Some(Commands::Delete { id }) => store.delete_task(id),
        Some(Commands::Swap { id1, id2 }) => store.swap_tasks(id1, id2),
        Some(Commands::Reset { force }) => store.reset_tasks(force),
        Some(Commands::Infos) => store.infos(),
        None => {}
    }
}
