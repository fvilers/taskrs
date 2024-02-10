use clap::{Parser, Subcommand};
use taskrs::prelude::*;

#[derive(Parser)]
#[command(about = "A simple command line to-do manager")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
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

    match cli.command {
        Some(Commands::Add { task }) => add_task(task),
        Some(Commands::List { all }) => list_tasks(all),
        Some(Commands::Update { id, task }) => update_task(id, task),
        Some(Commands::Done { id }) => mark_task(id, true),
        Some(Commands::Undone { id }) => mark_task(id, false),
        Some(Commands::Delete { id }) => delete_task(id),
        Some(Commands::Swap { id1, id2 }) => swap_tasks(id1, id2),
        Some(Commands::Reset { force }) => reset_tasks(force),
        Some(Commands::Infos) => infos(),
        None => {}
    }
}
