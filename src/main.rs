use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::{ColoredString, Colorize};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    fs::{File, OpenOptions},
    io::{self, BufReader, BufWriter},
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
        Some(Commands::Reset { force }) => reset(force),
        Some(Commands::Infos) => infos(),
        None => {}
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct TaskItem {
    id: u32,
    task: String,
    done: bool,
}

impl TaskItem {
    const fn new(id: u32, task: String) -> Self {
        Self {
            id,
            task,
            done: false,
        }
    }
}

impl Display for TaskItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let checkbox = if self.done { "🗹" } else { "☐" };
        write!(f, "{} {} {}", self.id, checkbox, self.task)
    }
}

fn read_tasks<P: AsRef<Path>>(path: P) -> Result<Vec<TaskItem>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let tasks = serde_json::from_reader(reader)?;

    Ok(tasks)
}

fn write_tasks<P: AsRef<Path>>(path: P, tasks: &[TaskItem]) -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    let writer = BufWriter::new(file);

    Ok(serde_json::to_writer(writer, &tasks)?)
}

const DEFAULT_FILENAME: &str = "c:\\Users\\Fabian\\tasks.json";

fn add_task(task: impl Into<String>) {
    let mut tasks = read_tasks(DEFAULT_FILENAME).unwrap_or_default();
    let max_id = tasks.iter().map(|task| task.id).max().unwrap_or(0);
    let new_task = TaskItem::new(max_id + 1, task.into());

    tasks.push(new_task);

    if write_tasks(DEFAULT_FILENAME, &tasks).is_err() {
        eprintln!("Could not write to {DEFAULT_FILENAME}");
    }
}

fn colorize_task(task: &TaskItem) -> ColoredString {
    if task.done {
        task.to_string().dimmed()
    } else {
        task.to_string().normal()
    }
}

fn list_tasks(all: bool) {
    let mut tasks = read_tasks(DEFAULT_FILENAME).unwrap_or_default();
    tasks.sort_by_key(|task| task.id);

    for task in tasks.iter().filter(|t| !t.done || all) {
        println!("{}", colorize_task(task));
    }
}

fn update_task(id: u32, task: impl Into<String>) {
    let mut tasks = read_tasks(DEFAULT_FILENAME).unwrap_or_default();
    let Some(current) = tasks.iter_mut().find(|task| task.id == id) else {
        eprintln!("Task not found");
        return;
    };

    current.task = task.into();

    if write_tasks(DEFAULT_FILENAME, &tasks).is_err() {
        eprintln!("Could not write to {DEFAULT_FILENAME}");
    }
}

fn mark_task(id: u32, done: bool) {
    let mut tasks = read_tasks(DEFAULT_FILENAME).unwrap_or_default();
    let Some(current) = tasks.iter_mut().find(|task| task.id == id) else {
        eprintln!("Task not found");
        return;
    };

    current.done = done;

    if write_tasks(DEFAULT_FILENAME, &tasks).is_err() {
        eprintln!("Could not write to {DEFAULT_FILENAME}");
    }
}

fn delete_task(id: u32) {
    let mut tasks = read_tasks(DEFAULT_FILENAME).unwrap_or_default();
    let Some(index) = tasks.iter().position(|task| task.id == id) else {
        eprintln!("Task not found");
        return;
    };

    tasks.remove(index);

    if write_tasks(DEFAULT_FILENAME, &tasks).is_err() {
        eprintln!("Could not write to {DEFAULT_FILENAME}");
    }
}

fn swap_tasks(id1: u32, id2: u32) {
    let mut tasks = read_tasks(DEFAULT_FILENAME).unwrap_or_default();
    let Some(index1) = tasks.iter().position(|task| task.id == id1) else {
        eprintln!("Task 1 not found");
        return;
    };
    let Some(index2) = tasks.iter().position(|task| task.id == id2) else {
        eprintln!("Task 2 not found");
        return;
    };

    tasks[index1].id = id2;
    tasks[index2].id = id1;

    if write_tasks(DEFAULT_FILENAME, &tasks).is_err() {
        eprintln!("Could not write to {DEFAULT_FILENAME}");
    }
}

fn pluralize(value: usize, singular: &str, plural: &str) -> String {
    format!(
        "{value} {}",
        match value {
            0 | 1 => singular,
            _ => plural,
        }
    )
}

fn reset(force: bool) {
    let mut tasks = read_tasks(DEFAULT_FILENAME).unwrap_or_default();

    if tasks.is_empty() {
        return;
    }

    let truncate = force || {
        println!(
            "Are your sure you want to permanently delete {} (y/N)?",
            pluralize(tasks.len(), "task", "tasks")
        );

        let mut input = String::new();

        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Could not read user input");
            return;
        }

        input.to_lowercase().trim() == "y"
    };

    if truncate {
        tasks.truncate(0);
    }

    if write_tasks(DEFAULT_FILENAME, &tasks).is_err() {
        eprintln!("Could not write to {DEFAULT_FILENAME}");
    }
}

fn infos() {
    let tasks = read_tasks(DEFAULT_FILENAME).unwrap_or_default();
    let done = tasks.iter().filter(|task| task.done).count();
    let remaining = tasks.len() - done;

    println!("File location: {DEFAULT_FILENAME}");
    println!("Done tasks: {done}");
    println!("Remaining tasks: {remaining}");
    println!("Total tasks: {}", tasks.len());
}
