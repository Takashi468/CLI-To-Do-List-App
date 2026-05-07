mod check_command;
mod writer_file;
mod list_to_do;
mod update_status;
mod help_command;

use check_command::check_command;
use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    println!("Welcome to the To-Do List App!");
    println!("Type 'help' to see the list of available commands.");

    loop {
        print!("Enter a command: ");
        io::stdout().flush().expect("Failed to flush");

        io::stdin().read_line(&mut input).expect("Failed to read line");

        if input.trim() == "exit" {
            break;
        }

        check_command(&input.trim());

        input.clear();
    }
}
