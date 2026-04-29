// use std::fs;
// use std::io::Write;

// const FILE_PATH: &str = "./task_list.json";

// struct format{
//     todo: String,
//     data: Option<String>,
//     status: Option<String>,
// }

// pub fn check_and_create_file() {

//     if fs::exists(FILE_PATH).expect("Can't check if file exists") {
//         println!("File already exists.");
//     } else {
//         fs::write(FILE_PATH, "").expect("Can't create file");
//     }
// }

// pub fn write_task(task: &str) {

//     if fs::exists(FILE_PATH).expect("Can't check if file exists") {
//         let mut file = fs::OpenOptions::new()
//             .write(true)
//             .append(true)
//             .open(FILE_PATH)
//             .expect("Can't open file");
//         writeln!(file, "{}", task).expect("Can't write to file");
//     } else {
//         check_and_create_file();
//         write_task(task);
//     }
// }


use std::fs;
use std::io::Write;
use serde_json;
use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use chrono::{Duration, Utc};

const FILE_PATH: &str = "./task_list.json";

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Task {
    pub id: u32,
    pub todo: String,
    pub data: Option<String>,
    pub status: Option<String>,
}

pub fn check_and_create_file() {

    if fs::exists(FILE_PATH).expect("Can't check if file exists") {
        println!("File already exists.");
    } else {
        fs::write(FILE_PATH, "").expect("Can't create file");
    }
}

pub fn save_task(task: &str) {
    let mut tasks: Vec<Task> = if fs::metadata(FILE_PATH).is_ok() {
        let data = fs::read_to_string(FILE_PATH).expect("Can't read file");
        serde_json::from_str(&data).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    };

    let parts: Vec<&str> = task.split(",").map(|s| s.trim()).collect();
    let mut task_date = Utc::now().format("%d-%m-%Y").to_string();

    if let Some(date_input) = parts.get(1) {
        let input_lower = date_input.to_lowercase();

        if input_lower.contains("day") {
            let days_count: i64 = input_lower
                .split_whitespace()
                .next()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
            if let Some(future_date) = Utc::now().checked_add_signed(chrono::Duration::days(days_count)) {
                task_date = future_date.format("%d-%m-%Y").to_string();

            }else if !date_input.is_empty(){
                task_date = date_input.to_string();
            }
        }

        if !parts.is_empty() && !parts[0].is_empty(){
            let max_id = tasks.iter().map(|t| t.id).max().unwrap_or(0);
            let new_task = Task {
                id: max_id + 1,
                todo: parts[0].to_string(),
                data: Some(task_date),
                status: Some("⏳ Pending".to_string()),
            };
            tasks.push(new_task)
        }else {
            println!("Invalid task format. Please provide a task in the format: <todo> <data>");
            return;
        }

    }

    // if parts.len() >= 1 {
    //     let max_id = tasks.iter().map(|t| t.id).max().unwrap_or(0);
    //     let new_task = Task {
    //         id: max_id + 1,
    //         todo: parts[0].to_string(),
    //         data: Some(parts.get(1).map(|s| s.to_string()).unwrap_or(now_ster)),
    //         status: Some("⏳ Pending".to_string()),
    //     };
    //     tasks.push(new_task);
    // }else{
    //     println!("Invalid task format. Please provide a task in the format: <todo> <data>");
    //     return;
    // }

    let json_data = serde_json::to_string_pretty(&tasks).expect("แปลงเป็น JSON ไม่ได้");
    let mut file = fs::File::create(FILE_PATH).expect("Can't create file");
    writeln!(file, "{}", json_data).expect("Can't write to file");
}
