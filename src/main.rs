use std::{env, fs};
use std::str::Split;

use bienenstock::Bienenstock;

fn main() {
    let mut bienenstock = Bienenstock::build();

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let script = fs::read_to_string(file_path)
        .expect("Invalid File Path");
    let commands = script.split(".");

    for command in commands {
        interpret_command(&mut bienenstock, command)
    }

    // bienenstock.print_all();
}

fn interpret_command(bienenstock: &mut Bienenstock, command: &str) {
    let mut params = command.trim().split(" ");

    match params.next() {
        // Waben-Command
        Some("Wabe") => bienenstock.rename(
                params.next()
                    .expect("Syntax Error")
                    .parse()
                    .expect("Syntax Error"),
                command.split("\"").nth(1)
                    .expect("Syntax Error")
            ),
        Some("Biene") => interpret_aufgabe(bienenstock, params),
        Some("So!") => (),
        Some(param) => panic!("Unknown Command: '{}'", param),
        _ => ()
    }
}

fn interpret_aufgabe(bienenstock: &mut Bienenstock, mut params: Split<&str>) {
    let biene_index: usize = params.next()
        .expect("Syntax Error").split(",").next()
        .expect("Syntax Error").parse()
        .expect("Syntax Error");

    match params.next() {
        Some("tanze") => tanzen(bienenstock, params, biene_index),
        Some(param) => panic!("Unknown Command: '{}'", param),
        _ => ()
    }
}

fn tanzen(bienenstock: &mut Bienenstock, mut params: Split<&str>, biene_index: usize) {
    if params.next() != Some("den") { panic!("Syntax Error"); }

    let name: bool = match params.next() {
        Some("Namen") => true,
        Some("Honig") => false,
        _ => panic!("Syntax Error")
    };

    if params.next() != Some("von") { panic!("Syntax Error"); }
    if params.next() != Some("Wabe") { panic!("Syntax Error"); }

    let wabe_index: usize = params.next()
        .expect("Syntax Error").parse()
        .expect("Syntax Error");

    bienenstock.tanzen(biene_index, wabe_index, name);
}