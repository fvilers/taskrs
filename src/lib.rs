use anyhow::Result;
use colored::{ColoredString, Colorize};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    fs::{File, OpenOptions},
    io::{self, BufReader, BufWriter},
    path::{Path, PathBuf},
};

pub mod prelude {
    pub use crate::{
        add_task, delete_task, infos, list_tasks, mark_task, reset_tasks, swap_tasks, update_task,
    };
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

pub fn add_task(task: impl Into<String>, file_name: PathBuf) {
    let mut tasks = read_tasks(&file_name).unwrap_or_default();
    let max_id = tasks.iter().map(|task| task.id).max().unwrap_or(0);
    let new_task = TaskItem::new(max_id + 1, task.into());

    tasks.push(new_task);

    if write_tasks(&file_name, &tasks).is_err() {
        eprintln!("Could not write to {}", file_name.display());
    }
}

fn colorize_task(task: &TaskItem) -> ColoredString {
    if task.done {
        task.to_string().dimmed()
    } else {
        task.to_string().normal()
    }
}

pub fn list_tasks(all: bool, file_name: PathBuf) {
    let mut tasks = read_tasks(file_name).unwrap_or_default();
    tasks.sort_by_key(|task| task.id);

    for task in tasks.iter().filter(|t| !t.done || all) {
        println!("{}", colorize_task(task));
    }
}

pub fn update_task(id: u32, task: impl Into<String>, file_name: PathBuf) {
    let mut tasks = read_tasks(&file_name).unwrap_or_default();
    let Some(current) = tasks.iter_mut().find(|task| task.id == id) else {
        eprintln!("Task not found");
        return;
    };

    current.task = task.into();

    if write_tasks(&file_name, &tasks).is_err() {
        eprintln!("Could not write to {}", file_name.display());
    }
}

pub fn mark_task(id: u32, done: bool, file_name: PathBuf) {
    let mut tasks = read_tasks(&file_name).unwrap_or_default();
    let Some(current) = tasks.iter_mut().find(|task| task.id == id) else {
        eprintln!("Task not found");
        return;
    };

    current.done = done;

    if write_tasks(&file_name, &tasks).is_err() {
        eprintln!("Could not write to {}", file_name.display());
    }
}

pub fn delete_task(id: u32, file_name: PathBuf) {
    let mut tasks = read_tasks(&file_name).unwrap_or_default();
    let Some(index) = tasks.iter().position(|task| task.id == id) else {
        eprintln!("Task not found");
        return;
    };

    tasks.remove(index);

    if write_tasks(&file_name, &tasks).is_err() {
        eprintln!("Could not write to {}", file_name.display());
    }
}

pub fn swap_tasks(id1: u32, id2: u32, file_name: PathBuf) {
    let mut tasks = read_tasks(&file_name).unwrap_or_default();
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

    if write_tasks(&file_name, &tasks).is_err() {
        eprintln!("Could not write to {}", file_name.display());
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

pub fn reset_tasks(force: bool, file_name: PathBuf) {
    let mut tasks = read_tasks(&file_name).unwrap_or_default();

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

    if write_tasks(&file_name, &tasks).is_err() {
        eprintln!("Could not write to {}", file_name.display());
    }
}

pub fn infos(file_name: PathBuf) {
    let tasks = read_tasks(&file_name).unwrap_or_default();
    let done = tasks.iter().filter(|task| task.done).count();
    let remaining = tasks.len() - done;

    println!("File location: {}", file_name.display());
    println!("Done tasks: {done}");
    println!("Remaining tasks: {remaining}");
    println!("Total tasks: {}", tasks.len());
}
