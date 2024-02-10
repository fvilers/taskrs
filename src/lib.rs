use anyhow::Result;
use colored::{ColoredString, Colorize};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    fs::{File, OpenOptions},
    io::{self, BufReader, BufWriter},
    path::{Path, PathBuf},
};

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

pub struct TaskStore {
    path: PathBuf,
}

impl TaskStore {
    #[must_use]
    pub const fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn add_task(&self, task: impl Into<String>) {
        let mut tasks = read_tasks(&self.path).unwrap_or_default();
        let max_id = tasks.iter().map(|task| task.id).max().unwrap_or(0);
        let new_task = TaskItem::new(max_id + 1, task.into());

        tasks.push(new_task);

        if write_tasks(&self.path, &tasks).is_err() {
            eprintln!("Could not write to {}", &self.path.display());
        }
    }

    pub fn list_tasks(&self, all: bool) {
        let mut tasks = read_tasks(&self.path).unwrap_or_default();
        tasks.sort_by_key(|task| task.id);

        for task in tasks.iter().filter(|t| !t.done || all) {
            println!("{}", colorize_task(task));
        }
    }

    pub fn update_task(&self, id: u32, task: impl Into<String>) {
        let mut tasks = read_tasks(&self.path).unwrap_or_default();
        let Some(current) = tasks.iter_mut().find(|task| task.id == id) else {
            eprintln!("Task not found");
            return;
        };

        current.task = task.into();

        if write_tasks(&self.path, &tasks).is_err() {
            eprintln!("Could not write to {}", &self.path.display());
        }
    }

    pub fn mark_task(&self, id: u32, done: bool) {
        let mut tasks = read_tasks(&self.path).unwrap_or_default();
        let Some(current) = tasks.iter_mut().find(|task| task.id == id) else {
            eprintln!("Task not found");
            return;
        };

        current.done = done;

        if write_tasks(&self.path, &tasks).is_err() {
            eprintln!("Could not write to {}", &self.path.display());
        }
    }

    pub fn delete_task(&self, id: u32) {
        let mut tasks = read_tasks(&self.path).unwrap_or_default();
        let Some(index) = tasks.iter().position(|task| task.id == id) else {
            eprintln!("Task not found");
            return;
        };

        tasks.remove(index);

        if write_tasks(&self.path, &tasks).is_err() {
            eprintln!("Could not write to {}", &self.path.display());
        }
    }

    pub fn swap_tasks(&self, id1: u32, id2: u32) {
        let mut tasks = read_tasks(&self.path).unwrap_or_default();
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

        if write_tasks(&self.path, &tasks).is_err() {
            eprintln!("Could not write to {}", &self.path.display());
        }
    }

    pub fn reset_tasks(&self, force: bool) {
        let mut tasks = read_tasks(&self.path).unwrap_or_default();

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

        if write_tasks(&self.path, &tasks).is_err() {
            eprintln!("Could not write to {}", &self.path.display());
        }
    }

    pub fn infos(&self) {
        let tasks = read_tasks(&self.path).unwrap_or_default();
        let done = tasks.iter().filter(|task| task.done).count();
        let remaining = tasks.len() - done;

        println!("File location: {}", &self.path.display());
        println!("Done tasks: {done}");
        println!("Remaining tasks: {remaining}");
        println!("Total tasks: {}", tasks.len());
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

fn colorize_task(task: &TaskItem) -> ColoredString {
    if task.done {
        task.to_string().dimmed()
    } else {
        task.to_string().normal()
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
