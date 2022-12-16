use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let script = fs::read_to_string(file_path)
        .expect("Invalid File Path");
    let commands = script.split(".");

    for command in commands {
        println!("{}", command.trim());
        let mut params = command.split(" ").into_iter();

        match params.next() {
            Some("Wabe") => println!("Wambe command"),
            Some("Biene") => println!("Biene command"),
            _ => println!("Unknown Command exception")
        }
    }
}