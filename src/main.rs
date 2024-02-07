use anyhow::Result;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter},
    path::Path,
};

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
        None => {}
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    task: String,
    done: bool,
}

impl Task {
    fn new(id: u32, task: String) -> Self {
        Task {
            id,
            task,
            done: false,
        }
    }
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

fn write_tasks<P: AsRef<Path>>(path: P, tasks: Vec<Task>) -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)?;
    let writer = BufWriter::new(file);

    Ok(serde_json::to_writer(writer, &tasks)?)
}

const DEFAULT_FILENAME: &str = "c:\\Users\\Fabian\\tasks.json";

fn add_task(task: impl Into<String>) {
    let mut tasks = read_tasks(DEFAULT_FILENAME).unwrap_or(Vec::new());
    let max_id = tasks.iter().map(|task| task.id).max().unwrap_or(0);
    let new_task = Task::new(max_id + 1, task.into());

    tasks.push(new_task);

    if write_tasks(DEFAULT_FILENAME, tasks).is_err() {
        eprintln!("Could not write to {DEFAULT_FILENAME}")
    }
}

fn list_tasks(all: bool) {
    let mut tasks = read_tasks(DEFAULT_FILENAME).unwrap_or(Vec::new());
    tasks.sort_by_key(|task| task.id);

    for task in tasks.iter().filter(|t| !t.done || all) {
        println!("{task}");
    }
}

fn update_task(id: u32, task: impl Into<String>) {
    let mut tasks = read_tasks(DEFAULT_FILENAME).unwrap_or(Vec::new());
    let Some(current) = tasks.iter_mut().find(|task| task.id == id) else {
        eprintln!("Task not found");
        return;
    };

    current.task = task.into();

    if write_tasks(DEFAULT_FILENAME, tasks).is_err() {
        eprintln!("Could not write to {DEFAULT_FILENAME}")
    }
}

fn mark_task(id: u32, done: bool) {
    let mut tasks = read_tasks(DEFAULT_FILENAME).unwrap_or(Vec::new());
    let Some(current) = tasks.iter_mut().find(|task| task.id == id) else {
        eprintln!("Task not found");
        return;
    };

    current.done = done;

    if write_tasks(DEFAULT_FILENAME, tasks).is_err() {
        eprintln!("Could not write to {DEFAULT_FILENAME}")
    }
}

fn delete_task(id: u32) {
    let mut tasks = read_tasks(DEFAULT_FILENAME).unwrap_or(Vec::new());
    let Some(index) = tasks.iter().position(|task| task.id == id) else {
        eprintln!("Task not found");
        return;
    };

    tasks.remove(index);

    if write_tasks(DEFAULT_FILENAME, tasks).is_err() {
        eprintln!("Could not write to {DEFAULT_FILENAME}")
    }
}
