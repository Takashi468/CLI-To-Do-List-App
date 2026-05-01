use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Serialize)]
struct Task {
    id: u32,
    todo: String,
    data: String,
    status: String,
}

pub fn update_task_status(task_id: &str, new_status: &str) {
    let target_id: u32 = match task_id.parse(){
        Ok(id) => id,
        Err(_) => {
            println!("Error: invalid task ID");
            return;
        }
    };

    let file_path = "./task_list.json";
    let data = fs::read_to_string(file_path).expect("Unable to read file");

    let mut tasks: Vec<Task> = serde_json::from_str(&data).expect("Unable to parse JSON");

    let mut found = false;
    for task in tasks.iter_mut() {
        if task.id == target_id {
            let input_lower = new_status.to_lowercase();
            if input_lower.contains("done") {
                task.status = String::from("✅ Done");
            } else {
                task.status = String::from("Not Done");
            }
            // task.status = String::from(new_status);
            found = true;
        }
    }

    if !found {
        println!("Task with ID {} not found", target_id);
        return;
    }

    // 5. แปลงกลับเป็น JSON และเขียนลงไฟล์ (Serialization)
    let updated_json = serde_json::to_string_pretty(&tasks).expect("Failed to serialize");
    fs::write(file_path, updated_json).expect("Unable to write file");

    println!("Task {} status updated to '{}' successfully!", target_id, new_status);
}
