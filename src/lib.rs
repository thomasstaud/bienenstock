use std::{
    io,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

struct Biene {
    name: String,
    honig: u32,
}

struct Wabe {
    name: String,
    honig: u32,
}

// executes bienenstock code
pub fn run(commands: Vec<&str>) {
    let sleep_secs = 1;

    let mut biene_handles = vec![];
    let biene_busy = Arc::new(Mutex::new([false; 1000]));

    // generate 1000 bienen
    let bienen = Arc::new(Mutex::new(vec![]));
    for _ in 0..1000 {
        bienen.lock().unwrap().push(Biene {
            name: String::new(),
            honig: 0,
        })
    }

    // generate 1000 waben
    let waben = Arc::new(Mutex::new(vec![]));
    for _ in 0..1000 {
        waben.lock().unwrap().push(Wabe {
            name: String::new(),
            honig: 0,
        })
    }

    for mut command in commands {
        let mut var_blocks = command.split("[").into_iter();
        var_blocks.next();

        let mut new_command = command.to_owned();
        for var_block in var_blocks {
            let variable = &var_block[..var_block.find("]").unwrap()];

            let mut params = variable.split(" ");

            let biene: bool = match params.nth(1) {
                Some("Biene") => true,
                Some("Wabe") => false,
                _ => panic!("Syntax Error"),
            };
            let index: usize = params.next().unwrap().parse().unwrap();

            let replace_from = "[".to_owned() + variable + "]";
            let replace_to = match biene {
                true => bienen.lock().unwrap()[index].honig,
                false => waben.lock().unwrap()[index].honig,
            }
            .to_string();

            new_command = str::replace(&command, &replace_from, &replace_to);
        }
        command = &new_command;

        // DEBUG
        // println!("{command}");

        let mut params = command.split(" ");
        // execute code based on command
        match params.next().unwrap() {
            "Wabe" => {
                // CMD Wabe X heißt "Y".
                // rename wabe
                let wabe_index: usize = params.next().unwrap().parse().unwrap();
                let wabe_name: &str = command.split("\"").nth(1).unwrap();
                waben.lock().unwrap()[wabe_index].name = wabe_name.to_string();
            }
            "Biene" => {
                // CMD Biene X, ...
                // find biene index
                let biene_index: usize = params
                    .next()
                    .unwrap()
                    .split(",")
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap()
                    .clone();

                if biene_busy.lock().unwrap()[biene_index] {
                    // crash if biene is already doing something
                    panic!("Biene {biene_index} is busy!");
                } else {
                    // set biene to busy
                    biene_busy.lock().unwrap()[biene_index] = true;
                }

                match params.next().unwrap() {
                    "tanze" => {
                        // CMD Biene X, tanze den Y von Wabe Z.
                        // check what should be printed
                        let name: bool = match params.nth(1) {
                            Some("Namen") => true,
                            Some("Honig") => false,
                            _ => panic!("Syntax Error"),
                        };

                        let wabe_index: usize = params.nth(2).unwrap().parse().unwrap();

                        // store whatever should be printed
                        let print = match name {
                            true => waben.lock().unwrap()[wabe_index].name.to_owned(),
                            false => waben.lock().unwrap()[wabe_index].honig.to_string(),
                        };

                        // spawn thread for biene to tanz
                        let biene_busy = Arc::clone(&biene_busy);
                        let handle = thread::spawn(move || {
                            thread::sleep(Duration::from_secs(sleep_secs));
                            println!("{print}");
                            biene_busy.lock().unwrap()[biene_index] = false;
                        });
                        biene_handles.push(handle);
                    }
                    "hole" => {
                        // CMD Biene X, hole Y Nektar.
                        // oder:
                        // CMD Biene X, hole Y Honig von Wabe Z
                        let amount: u32 = params.next().unwrap().parse().unwrap();

                        match params.next() {
                            Some("Nektar") => {
                                // spawn thread for biene to hol
                                let biene_busy = Arc::clone(&biene_busy);
                                let bienen = Arc::clone(&bienen);
                                let handle = thread::spawn(move || {
                                    thread::sleep(Duration::from_secs(sleep_secs));
                                    bienen.lock().unwrap()[biene_index].honig += amount;
                                    biene_busy.lock().unwrap()[biene_index] = false;
                                });
                                biene_handles.push(handle);
                            }
                            Some("Honig") => {
                                let wabe_index: usize = params.nth(2).unwrap().parse().unwrap();

                                // spawn thread for biene to hol
                                let biene_busy = Arc::clone(&biene_busy);
                                let bienen = Arc::clone(&bienen);
                                let waben = Arc::clone(&waben);
                                let handle = thread::spawn(move || {
                                    thread::sleep(Duration::from_secs(sleep_secs));
                                    waben.lock().unwrap()[wabe_index].honig -= amount;
                                    bienen.lock().unwrap()[biene_index].honig += amount;
                                    biene_busy.lock().unwrap()[biene_index] = false;
                                });
                                biene_handles.push(handle);
                            }
                            _ => panic!("Syntax Error"),
                        }
                    }
                    "sammle" => {
                        // CMD Biene X, sammle Y vom Benutzer.
                        // sammle input from user
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).unwrap();

                        let name: bool = match params.next() {
                            Some("Namen") => true,
                            Some("Nektar") => false,
                            _ => panic!("Syntax Error"),
                        };

                        params.nth(1);

                        // spawn thread for biene to sammel
                        let biene_busy = Arc::clone(&biene_busy);
                        let bienen = Arc::clone(&bienen);
                        let handle = thread::spawn(move || {
                            thread::sleep(Duration::from_secs(sleep_secs));
                            match name {
                                true => {
                                    bienen.lock().unwrap()[biene_index].name =
                                        input.trim().to_string()
                                }
                                false => {
                                    bienen.lock().unwrap()[biene_index].honig =
                                        input.trim().parse().unwrap()
                                }
                            };
                            biene_busy.lock().unwrap()[biene_index] = false;
                        });
                        biene_handles.push(handle);
                    }
                    "bringe" => {
                        // CMD Biene X, bringe Y zu Wabe Z.
                        // save name or honig to wabe
                        let name: bool = match params.next() {
                            Some("Namen") => true,
                            Some("Honig") => false,
                            _ => panic!("Syntax Error"),
                        };

                        let wabe_index: usize = params.nth(2).unwrap().parse().unwrap();

                        // spawn thread for biene to bring
                        let biene_busy = Arc::clone(&biene_busy);
                        let bienen = Arc::clone(&bienen);
                        let waben = Arc::clone(&waben);
                        let handle = thread::spawn(move || {
                            thread::sleep(Duration::from_secs(sleep_secs));
                            match name {
                                true => {
                                    waben.lock().unwrap()[wabe_index].name =
                                        bienen.lock().unwrap()[biene_index].name.to_owned()
                                }
                                false => {
                                    waben.lock().unwrap()[wabe_index].honig +=
                                        bienen.lock().unwrap()[biene_index].honig;
                                    bienen.lock().unwrap()[biene_index].honig = 0;
                                }
                            };
                            biene_busy.lock().unwrap()[biene_index] = false;
                        });
                        biene_handles.push(handle);
                    }
                    _ => panic!("Unknown Command: '{command}'"),
                }

                // CMD Biene X, ... und warte.
                match params.nth(1) {
                    Some("warte") => {
                        while biene_busy.lock().unwrap()[biene_index] {
                            thread::sleep(Duration::from_millis(1));
                        }
                    }
                    Some(_) => panic!("Unknown Command: {command}"),
                    None => (),
                }
            }
            "Warte" => {
                // CMD Warte auf Biene X.
                // wait for thread to end
                let biene_index: usize = params.nth(2).unwrap().parse().unwrap();
                while biene_busy.lock().unwrap()[biene_index] {
                    thread::sleep(Duration::from_millis(1));
                }
            }
            "<#!--:" => {
                if params.last().unwrap() != ":--!#>" {
                    panic!("Comment was not properly closed: {command}");
                }
            }
            "So!" => (),
            _ => panic!("Unknown Command: '{command}'"),
        }
    }

    for handle in biene_handles {
        handle.join().unwrap();
    }
}
