use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("In file {}", file_path);

    let script = fs::read_to_string(file_path)
        .expect("Invalid File Path");
    let commands = script.split(".");

    for command in commands {
        println!("{}", command.trim());
    }
}