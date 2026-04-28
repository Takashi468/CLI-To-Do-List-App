use std::fs;
use std::io;
use comfy_table::Table;

const FILE_PATH: &str = "./task_list.json";

pub fn list_task() {
    if fs::exists(FILE_PATH).expect("Failed to check if file exists") {
        let contents = fs::read_to_string(FILE_PATH).expect("Failed to read file");
        let work: Vec<&str> = contents.split_whitespace().collect();
        let work_out = work.join("\n");
        println!("{}", work_out);
        let mut table = Table::new();
        table.set_header(["Task", "Status"]);
        for task in work {
            table.add_row([task, "⏳ Pending"]);
        }
        println!("{}", table);
    } else {
        println!("No tasks found.");
    }
}
