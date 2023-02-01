use std::{
    io,
    process::exit,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

struct Biene {
    name: String,
    honig: u32,
}

struct Wabe {
    name: String,
    honig: u32,
}

fn error(msg: &str) -> ! {
    println!("{}", msg);
    exit(98101101)
}

// executes bienenstock code
pub fn run(lines: Vec<&str>) {
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

    let mut line_number = 0;
    let mut line: &str = lines.get(line_number).unwrap();

    loop {
        let mut command = line;

        // variables
        let mut var_blocks = command.split("[").into_iter();
        var_blocks.next();

        let mut tmp_command = command.to_owned();
        for var_block in var_blocks {
            let variable = &var_block[..var_block.find("]").unwrap()];

            let mut params = variable.split(" ");

            let value: u8 = match params.next() {
                Some("Name") => 0,
                Some("Honig") => 1,
                Some("Namenslänge") => 2,
                _ => error(&format!(
                    "Die Bienen verstehen die Zeile {} nicht.",
                    line_number + 1
                )),
            };

            let biene: bool = match params.next() {
                Some("Biene") => true,
                Some("Wabe") => false,
                _ => error(&format!(
                    "Die Bienen verstehen die Zeile {} nicht.",
                    line_number + 1
                )),
            };
            let index: usize = params.next().unwrap().parse().unwrap();

            let replace_from = "[".to_owned() + variable + "]";
            let replace_to = match value {
                0 => match biene {
                    true => bienen.lock().unwrap()[index - 1].name.to_owned(),
                    false => waben.lock().unwrap()[index - 1].name.to_owned(),
                },
                1 => match biene {
                    true => bienen.lock().unwrap()[index - 1].honig.to_string(),
                    false => waben.lock().unwrap()[index - 1].honig.to_string(),
                },
                2 => match biene {
                    true => bienen.lock().unwrap()[index - 1].name.len().to_string(),
                    false => waben.lock().unwrap()[index - 1].name.len().to_string(),
                },
                _ => panic!("AMBIGUOUS INTERNAL ERROR"),
            };

            tmp_command = str::replace(&command, &replace_from, &replace_to);
        }
        command = &tmp_command;

        // DEBUG
        // println!("{command}");

        let mut params = command.split(" ");
        // execute code based on command
        match params.next().unwrap() {
            "Wabe" => {
                // CMD Wabe A heißt "B".
                // rename wabe
                let wabe_index: usize = params.next().unwrap().parse().unwrap();
                let wabe_name: &str = command.split("\"").nth(1).unwrap();
                waben.lock().unwrap()[wabe_index - 1].name = wabe_name.to_string();
            }
            "Biene" => {
                // CMD Biene A, ...
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

                if biene_busy.lock().unwrap()[biene_index - 1] {
                    // crash if biene is already doing something
                    error(&format!(
                        "Biene {} kann in Zeile {} nicht einspringen, sie ist schon beschäftigt.",
                        biene_index,
                        line_number + 1
                    ));
                } else {
                    // set biene to busy
                    biene_busy.lock().unwrap()[biene_index - 1] = true;
                }

                match params.next().unwrap() {
                    "tanze" => {
                        // CMD Biene A, tanze den B von Wabe C.
                        // check what should be printed
                        let name: bool = match params.nth(1) {
                            Some("Namen") => true,
                            Some("Honig") => false,
                            _ => error(&format!(
                                "Die Biene versteht nicht, was sie in Zeile {} tanzen soll.",
                                line_number + 1
                            )),
                        };

                        let wabe_index: usize = params.nth(2).unwrap().parse().unwrap();

                        // store whatever should be printed
                        let print = match name {
                            true => waben.lock().unwrap()[wabe_index - 1].name.to_owned(),
                            false => waben.lock().unwrap()[wabe_index - 1].honig.to_string(),
                        };

                        // spawn thread for biene to tanz
                        let biene_busy = Arc::clone(&biene_busy);
                        let handle = thread::spawn(move || {
                            thread::sleep(Duration::from_secs(sleep_secs));
                            println!("{print}");
                            biene_busy.lock().unwrap()[biene_index - 1] = false;
                        });
                        biene_handles.push(handle);
                    }
                    "hole" => {
                        match params.next().unwrap() {
                            "Namen" => {
                                // CMD Biene A, hole Namen von Wabe C.
                                let wabe_index: usize = params.nth(2).unwrap().parse().unwrap();

                                // spawn thread for biene to hol
                                let biene_busy = Arc::clone(&biene_busy);
                                let bienen = Arc::clone(&bienen);
                                let waben = Arc::clone(&waben);
                                let handle = thread::spawn(move || {
                                    thread::sleep(Duration::from_secs(sleep_secs));
                                    bienen.lock().unwrap()[biene_index - 1].name =
                                        waben.lock().unwrap()[wabe_index - 1].name.to_owned();
                                    biene_busy.lock().unwrap()[biene_index - 1] = false;
                                });
                                biene_handles.push(handle);
                            }
                            "Buchstabe" => {
                                // CMD Biene A, hole Buchstabe B von Wabe C.
                                let buchstabe_index: usize =
                                    params.next().unwrap().parse().unwrap();
                                let wabe_index: usize = params.nth(2).unwrap().parse().unwrap();

                                // spawn thread for biene to hol
                                let biene_busy = Arc::clone(&biene_busy);
                                let bienen = Arc::clone(&bienen);
                                let waben = Arc::clone(&waben);
                                let handle = thread::spawn(move || {
                                    thread::sleep(Duration::from_secs(sleep_secs));
                                    let mut name =
                                        waben.lock().unwrap()[wabe_index - 1].name.to_owned();
                                    let buchstabe = name.remove(buchstabe_index - 1).to_string();
                                    bienen.lock().unwrap()[biene_index - 1].name = buchstabe;
                                    waben.lock().unwrap()[wabe_index - 1].name = name;
                                    biene_busy.lock().unwrap()[biene_index - 1] = false;
                                });
                                biene_handles.push(handle);
                            }
                            amount => {
                                // CMD Biene A, hole B Nektar.
                                // CMD Biene A, hole B Honig von Wabe C.
                                let amount: u32 = amount.parse().unwrap();

                                match params.next() {
                                    Some("Nektar") => {
                                        // spawn thread for biene to hol
                                        let biene_busy = Arc::clone(&biene_busy);
                                        let bienen = Arc::clone(&bienen);
                                        let handle = thread::spawn(move || {
                                            thread::sleep(Duration::from_secs(sleep_secs));
                                            bienen.lock().unwrap()[biene_index - 1].honig += amount;
                                            biene_busy.lock().unwrap()[biene_index - 1] = false;
                                        });
                                        biene_handles.push(handle);
                                    }
                                    Some("Honig") => {
                                        let wabe_index: usize =
                                            params.nth(2).unwrap().parse().unwrap();

                                        // spawn thread for biene to hol
                                        let biene_busy = Arc::clone(&biene_busy);
                                        let bienen = Arc::clone(&bienen);
                                        let waben = Arc::clone(&waben);
                                        let handle = thread::spawn(move || {
                                            thread::sleep(Duration::from_secs(sleep_secs));
                                            waben.lock().unwrap()[wabe_index - 1].honig -= amount;
                                            bienen.lock().unwrap()[biene_index - 1].honig += amount;
                                            biene_busy.lock().unwrap()[biene_index - 1] = false;
                                        });
                                        biene_handles.push(handle);
                                    }
                                    _ => error(&format!(
                                        "Die Biene versteht nicht, was sie in Zeile {} holen soll.",
                                        line_number + 1
                                    )),
                                }
                            }
                        }
                    }
                    "sammle" => {
                        // CMD Biene A, sammle B vom Benutzer.
                        // sammle input from user
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).unwrap();

                        let name: bool = match params.next() {
                            Some("Namen") => true,
                            Some("Nektar") => false,
                            _ => error(&format!(
                                "Die Biene versteht nicht, was sie in Zeile {} sammeln soll.",
                                line_number + 1
                            )),
                        };

                        params.nth(1);

                        // spawn thread for biene to sammel
                        let biene_busy = Arc::clone(&biene_busy);
                        let bienen = Arc::clone(&bienen);
                        let handle = thread::spawn(move || {
                            thread::sleep(Duration::from_secs(sleep_secs));
                            match name {
                                true => {
                                    bienen.lock().unwrap()[biene_index - 1].name =
                                        input.trim().to_string()
                                }
                                false => {
                                    bienen.lock().unwrap()[biene_index - 1].honig = input
                                        .trim()
                                        .parse()
                                        .expect("Die Bienen sind reichlich verwirrt.")
                                }
                            };
                            biene_busy.lock().unwrap()[biene_index - 1] = false;
                        });
                        biene_handles.push(handle);
                    }
                    "bringe" => {
                        // CMD Biene A, bringe B zu Wabe C.
                        // save name or honig to wabe
                        let name: bool = match params.next() {
                            Some("Namen") => true,
                            Some("Honig") => false,
                            _ => error(&format!(
                                "Die Biene versteht nicht, was sie in Zeile {} bringen soll.",
                                line_number + 1
                            )),
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
                                    waben.lock().unwrap()[wabe_index - 1].name +=
                                        &bienen.lock().unwrap()[biene_index - 1].name.to_owned()
                                }
                                false => {
                                    waben.lock().unwrap()[wabe_index - 1].honig +=
                                        bienen.lock().unwrap()[biene_index - 1].honig;
                                    bienen.lock().unwrap()[biene_index - 1].honig = 0;
                                }
                            };
                            biene_busy.lock().unwrap()[biene_index - 1] = false;
                        });
                        biene_handles.push(handle);
                    }
                    _ => error(&format!(
                        "Die Biene versteht nicht, was sie in Zeile {} machen soll.",
                        line_number + 1
                    )),
                }

                // CMD Biene A, ... und warte.
                match params.nth(1) {
                    Some("warte") => {
                        let start_wait = Instant::now();
                        while biene_busy.lock().unwrap()[biene_index - 1] {
                            thread::sleep(Duration::from_millis(1));
                            if start_wait.elapsed() > Duration::from_secs(10) {
                                error(&format!(
                                    "Die Bienen sind in Zeile {} reichlich verwirrt.",
                                    line_number + 1
                                ));
                            }
                        }
                    }
                    Some(_) => error(&format!(
                        "Die Bienen sind in Zeile {} reichlich verwirrt.",
                        line_number + 1
                    )),
                    None => (),
                }
            }
            "Starte" => {
                // CMD Starte die Choreografie X.
                let choreografie = "\"".to_string() + command.split("\"").nth(1).unwrap() + "\"";
                for (nr, line) in lines.iter().enumerate() {
                    // println!("no {}, {}", nr, line);
                    if line.contains(&choreografie) && line.starts_with("Hier") {
                        // println!("yes {}, {}", nr, line);
                        line_number = nr;
                        break;
                    }
                }
            }
            "Wenn" => {
                // CMD Wenn A B mehr Honig hat als C D, starte die Choreografie E.
                // CMD Wenn A B gleich viel Honig hat als C D, starte die Choreografie E.
                // CMD Wenn A B weniger Honig hat als C D, starte die Choreografie E.
                // CMD Wenn A B gleich heißt wie C D, starte die Choreografie E.
                let obj1_biene = match params.next().unwrap() {
                    "Biene" => true,
                    "Wabe" => false,
                    _ => error(&format!(
                        "Die Bienen sind in Zeile {} reichlich verwirrt.",
                        line_number + 1
                    )),
                };
                let obj1_index: usize = params.next().unwrap().parse().unwrap();

                let comparison = match params.next().unwrap() {
                    "gleich" => {
                        match params.next().unwrap() {
                            "viel" => {
                                // skip "Honig hat"
                                params.nth(1);
                                "gleich"
                            }
                            _ => "name",
                        }
                    }
                    comp => {
                        // skip "Honig hat"
                        params.nth(1);
                        comp
                    }
                };

                let obj2_biene = match params.nth(1).unwrap() {
                    "Biene" => true,
                    "Wabe" => false,
                    _ => error(&format!(
                        "Die Bienen sind in Zeile {} reichlich verwirrt.",
                        line_number + 1
                    )),
                };
                // remove colon from index
                let mut obj2_index = params.next().unwrap().to_string();
                obj2_index.pop().unwrap();
                let obj2_index: usize = obj2_index.parse().unwrap();

                match comparison {
                    "name" => {
                        // check if names are the same
                        let name1 = match obj1_biene {
                            true => bienen.lock().unwrap()[obj1_index - 1].name.to_owned(),
                            false => waben.lock().unwrap()[obj1_index - 1].name.to_owned(),
                        };

                        let name2 = match obj2_biene {
                            true => bienen.lock().unwrap()[obj2_index - 1].name.to_owned(),
                            false => waben.lock().unwrap()[obj2_index - 1].name.to_owned(),
                        };

                        if name1 == name2 {
                            // start choreografie
                            let choreografie =
                                "\"".to_string() + command.split("\"").nth(1).unwrap() + "\"";
                            for (nr, line) in lines.iter().enumerate() {
                                if line.contains(&choreografie) && line.starts_with("Hier") {
                                    line_number = nr;
                                    break;
                                }
                            }
                        }
                    }
                    _ => {
                        // compare honey values
                        let val1 = match obj1_biene {
                            true => bienen.lock().unwrap()[obj1_index - 1].honig,
                            false => waben.lock().unwrap()[obj1_index - 1].honig,
                        };

                        let val2 = match obj2_biene {
                            true => bienen.lock().unwrap()[obj2_index - 1].honig,
                            false => waben.lock().unwrap()[obj2_index - 1].honig,
                        };

                        // DEBUG
                        // println!("{val1} soll {comparison} sein wie {val2}.");

                        if match comparison {
                            "gleich" => val1 == val2,
                            "weniger" => val1 < val2,
                            "mehr" => val1 > val2,
                            _ => error(&format!(
                                "Die Bienen sind in Zeile {} reichlich verwirrt.",
                                line_number + 1
                            )),
                        } {
                            // start choreografie
                            let choreografie =
                                "\"".to_string() + command.split("\"").nth(1).unwrap() + "\"";
                            for (nr, line) in lines.iter().enumerate() {
                                if line.contains(&choreografie) && line.starts_with("Hier") {
                                    line_number = nr;
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            "Warte" => {
                // CMD Warte auf Biene A.
                // wait for thread to end
                let biene_index: usize = params.nth(2).unwrap().parse().unwrap();
                while biene_busy.lock().unwrap()[biene_index - 1] {
                    thread::sleep(Duration::from_millis(1));
                }
            }
            "Hier" => (),
            "<#!--:" => {
                if params.last().unwrap() != ":--!#>" {
                    error(&format!(
                        "Die Bienen sind in Zeile {} reichlich verwirrt.",
                        line_number + 1
                    ));
                }
            }
            "So!" => {
                break;
            }
            _ => error(&format!(
                "Die Bienen sind in Zeile {} reichlich verwirrt.",
                line_number + 1
            )),
        }

        line_number += 1;
        line = lines.get(line_number).unwrap();
    }

    for handle in biene_handles {
        handle.join().unwrap();
    }
}
