use std::fs;
use std::io;
use comfy_table::Table;
use crate::writer_file::Task;

const FILE_PATH: &str = "./task_list.json";

pub fn list_task() {
    if fs::exists(FILE_PATH).expect("Failed to check if file exists") {
        let contents = fs::read_to_string(FILE_PATH).expect("Failed to read file");

        let tasks: Vec<Task> = serde_json::from_str(&contents).unwrap_or_else(|_| vec![]);

        if tasks.is_empty() {
            println!("No tasks found.");
            return;
        }

        let mut table = Table::new();
        table.set_header(["ID", "TODO", "Date", "Status"]);
        for task in tasks {
            table.add_row(vec![
                task.id.to_string(),
                task.todo,
                task.data.unwrap_or_else(|| "N/A".to_string()),
                task.status.unwrap_or_else(|| "unknown".to_string()),
            ]);
        }
        println!("{}", table);
    } else {
        println!("No tasks found.");
    }
}
