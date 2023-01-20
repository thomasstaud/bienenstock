use core::time;
use std::thread;

// executes bienenstock code
pub fn run(commands: Vec<&str>) {
    let mut biene_handles = vec![];

    let mut biene_busy = [false; 1000];
    let mut wabe_names = [""; 1000];
    let mut wabe_honig = [0; 1000];

    for command in commands {
        let mut params = command.split(" ");
        // execute code based on command
        match params.next() {
            Some("Wabe") => {
                // rename wabe
                let wabe_index: usize = params.next().expect("Syntax Error")
                    .parse().expect("Syntax Error");
                let wabe_name: &str = command.split("\"").nth(1)
                    .expect("Syntax Error");
                wabe_names[wabe_index] = wabe_name;
            }
            Some("Biene") => {
                // tanze name of wabe
                let biene_index: usize = params.next().expect("Syntax Error")
                    .split(",").next().expect("Syntax Error")
                    .parse().expect("Syntax Error");

                if biene_busy[biene_index] {
                    // crash if biene is already doing something
                    panic!("Biene {biene_index} is busy!");
                } else {
                    if params.next() != Some("tanze") { panic!("Syntax Error"); }
                    if params.next() != Some("den") { panic!("Syntax Error"); }
                    if params.next() != Some("Namen") { panic!("Syntax Error"); }
                    if params.next() != Some("von") { panic!("Syntax Error"); }
                    if params.next() != Some("Wabe") { panic!("Syntax Error"); }

                    let wabe_index: usize = params.next().expect("Syntax Error")
                        .parse().expect("Syntax Error");
                    let wabe_name = String::from(wabe_names[wabe_index]);

                    // spawn thread for biene to tanz
                    let handle = thread::spawn(move || {
                        thread::sleep(time::Duration::from_secs(2));
                        println!("{wabe_name}");
                    });
                    biene_handles.push(handle);
                }

            }
            Some("So!") => println!("So!"),
            Some(param) => panic!("Unknown Command: '{}'", param),
            _ => ()
        }
    }

    for handle in biene_handles {
        handle.join().unwrap();
    }
}