use std::fs;
use std::io::Write;

const FILE_PATH: &str = "./task_list.json";

pub fn check_and_create_file() {

    if fs::exists(FILE_PATH).expect("Can't check if file exists") {
        println!("File already exists.");
    } else {
        fs::write(FILE_PATH, "").expect("Can't create file");
    }
}

pub fn write_task(task: &str) {

    if fs::exists(FILE_PATH).expect("Can't check if file exists") {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(FILE_PATH)
            .expect("Can't open file");
        writeln!(file, "{}", task).expect("Can't write to file");
    } else {
        check_and_create_file();
        write_task(task);
    }
}
