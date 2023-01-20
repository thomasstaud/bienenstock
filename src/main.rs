use std::{env, fs, time::Instant};

use bienenstock::run;

fn main() {
    let now = Instant::now();

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let script = fs::read_to_string(file_path).expect("Invalid File Path");
    let commands = script.split(".\r\n").collect();

    run(commands);

    let elapsed = now.elapsed();
    println!("Bienenstock ist fertig. Ausführdauer: {:.2?}", elapsed);
}
