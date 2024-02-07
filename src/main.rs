use anyhow::Result;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, fs::File, io::BufReader, path::Path};

#[derive(Parser)]
#[command(about = "A simple command line to-do manager")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "List tasks")]
    List {
        #[arg(short, long, help = "Include done tasks")]
        all: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::List { all }) => list_tasks(all),
        None => {}
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    task: String,
    done: bool,
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let checkbox = match self.done {
            true => "🗹",
            false => "☐",
        };
        write!(f, "{} {} {}", self.id, checkbox, self.task)
    }
}

fn read_tasks<P: AsRef<Path>>(path: P) -> Result<Vec<Task>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let tasks = serde_json::from_reader(reader)?;

    Ok(tasks)
}

const DEFAULT_FILENAME: &str = "c:\\Users\\Fabian\\tasks.json";

fn list_tasks(all: bool) {
    let mut tasks = read_tasks(DEFAULT_FILENAME).unwrap_or(Vec::new());
    tasks.sort_by_key(|task| task.id);

    for task in tasks.iter().filter(|t| !t.done || all) {
        println!("{task}");
    }
}
