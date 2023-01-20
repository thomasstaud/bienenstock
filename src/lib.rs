use std::{thread, time::Duration, sync::{Mutex, Arc}, io};

struct Biene {
    name: String,
    nektar: u32
}

struct Wabe {
    name: String,
    honig: u32
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
            nektar: 0
        })
    }

    // generate 1000 waben
    let waben = Arc::new(Mutex::new(vec![]));
    for _ in 0..1000 {
        waben.lock().unwrap().push(Wabe {
            name: String::new(),
            honig: 0
        })
    }

    for command in commands {
        let mut params = command.split(" ");
        // execute code based on command
        match params.next().unwrap() {
            "Wabe" => {
                // rename wabe
                let wabe_index: usize = params.next().unwrap().parse().unwrap();
                let wabe_name: &str = command.split("\"").nth(1).unwrap();
                waben.lock().unwrap()[wabe_index].name = wabe_name.to_string();
            },
            "Biene" => {
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
                    "sammle" => {
                        // sammle input from user
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).unwrap();

                        let name: bool = match params.next() {
                            Some("Namen") => true,
                            Some("Nektar") => false,
                            _ => panic!("Syntax Error"),
                        };

                        // spawn thread for biene to sammel
                        let biene_busy = Arc::clone(&biene_busy);
                        let bienen = Arc::clone(&bienen);
                        let handle = thread::spawn(move || {
                            thread::sleep(Duration::from_secs(sleep_secs));
                            match name {
                                true => bienen.lock().unwrap()[biene_index].name = input.trim().to_string(),
                                false => bienen.lock().unwrap()[biene_index].nektar = input.trim().parse().unwrap()
                            };
                            biene_busy.lock().unwrap()[biene_index] = false;
                        });
                        biene_handles.push(handle);
                    }
                    "bringe" => {
                        // save name or nektar to wabe
                        let name: bool = match params.next() {
                            Some("Namen") => true,
                            Some("Nektar") => false,
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
                                true => waben.lock().unwrap()[wabe_index].name = bienen.lock().unwrap()[biene_index].name.to_owned(),
                                false => waben.lock().unwrap()[wabe_index].honig += bienen.lock().unwrap()[biene_index].nektar
                            };
                            biene_busy.lock().unwrap()[biene_index] = false;
                        });
                        biene_handles.push(handle);
                    }
                    _ => panic!("Unknown Command: '{}'", command),
                }
            }
            "Warte" => {
                // wait for thread to end
                let biene_index: usize = params.nth(2).unwrap().parse().unwrap();
                while biene_busy.lock().unwrap()[biene_index] {
                    thread::sleep(Duration::from_millis(1));
                }
            }
            "So!" => (),
            _ => panic!("Unknown Command: '{}'", command),
        }
    }

    for handle in biene_handles {
        handle.join().unwrap();
    }
}
