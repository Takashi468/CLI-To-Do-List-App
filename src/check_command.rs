use crate::writer_file::*;
use std::io::Write;
use crate::list_to_do::list_task;


pub fn check_command(command: &str) {
    match command {
        "todo" => todo_command(),
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
