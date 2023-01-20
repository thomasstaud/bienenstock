use std::{env, fs};

use bienenstock::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let script = fs::read_to_string(file_path)
        .expect("Invalid File Path");
    let commands = script.split(".\r\n").collect();

    run(commands);
}