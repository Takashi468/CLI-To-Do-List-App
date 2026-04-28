mod check_command;
mod writer_file;
mod list_to_do;
use check_command::check_command;
use std::io::{self, Write};

fn main() {
    let mut input = String::new();

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
