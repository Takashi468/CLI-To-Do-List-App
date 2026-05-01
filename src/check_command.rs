use crate::writer_file::*;
use std::io::Write;
use crate::list_to_do::list_task;
use crate::update_status::update_task_status;


pub fn check_command(full_input: &str) {
    let parts: Vec<&str> = full_input.trim().split_whitespace().collect();

    if parts.is_empty() {
        return;
    }

    let command = parts[0];
  //  let args = &parts[1];
//    let status = &parts[2];

    match command {
        "todo" => {
            if parts.len() >= 3 {
                let id = parts[1];
                let status = parts[2];
                update_task_status(id, status);
            }else {
                todo_command();
            }
            // if args.is_empty() {
            //     todo_command();
            // } else {
            //     update_task_status(args, status);
            // }
        },
        "list" => list_task(),
        "end" => return,
        _ => println!("Invalid command: {}", command),
    }
}
fn todo_command() {
    let mut input = String::new();

    print!("Enter a task: ");
    std::io::stdout().flush().expect("Failed to flush stdout");
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    let work: Vec<&str> = input.split_whitespace().collect();
    let outtask = work.join("\n");

    // println!("Task added:{}", input.replace(" ", "\n"));
    println!("Task added:{}", input);
    // write_task(&outtask);
    save_task(&outtask);
}
