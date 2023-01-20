use aufgaben::*;

mod aufgaben;

struct Biene {
    nektar: u32,
    aufgabe: Box<dyn Aufgabe>
}

struct Wabe {
    name: String,
    honig: u32
}

pub struct Bienenstock {
    bienen: Vec<Biene>,
    waben: Vec<Wabe>
}

impl Bienenstock {
    pub fn build() -> Self {
        let mut bienenstock = Bienenstock {
            bienen: Vec::new(),
            waben: Vec::new()
        };

        // generate 1000 bienen and waben
        for _ in 0..1000 {
            bienenstock.bienen.push(Biene {
                nektar: 0,
                aufgabe: Box::new(Leerlauf {})
            });

            bienenstock.waben.push(Wabe {
                name: String::new(),
                honig: 0
            });
        }

        bienenstock
    }

    pub fn rename(&mut self, wabe_index: usize, name: &str) {
        match self.waben.get_mut(wabe_index - 1) {
            Some(wabe) => wabe.name = name.to_string(),
            None => panic!("Wabe {wabe_index} existiert nicht")
        }
    }

    pub fn tanzen(&mut self, biene_index: usize, wabe_index: usize, name: bool) {
        // edit aufgabe -> mutable
        let biene = match self.bienen.get_mut(biene_index - 1) {
            Some(biene) => biene,
            None => panic!("Biene {biene_index} existiert nicht")
        };
        biene.aufgabe = Box::new(Tanzen { name, wabe_index });

        // execute aufgabe -> immutable
        let biene = self.bienen.get(biene_index - 1).unwrap();
        biene.aufgabe.execute(&self);
    }

    pub fn print(&self, wabe_index: usize, name: bool) {
        let wabe = self.waben.get(wabe_index - 1).unwrap();

        if name {
            println!("{}", wabe.name);
        } else {
            println!("{}", wabe.honig);
        }
    }

    pub fn print_all(&self) {
        // print all bienen with nektar or aufgabe
        for (i, biene) in self.bienen.iter().enumerate() {
            if biene.nektar != 0 {
                println!("Biene {i} hat {} Nektar", biene.nektar);
            }

            if let Some(str) = biene.aufgabe.to_string() {
                println!("Biene {i} {str}");
            }
        }

        // print all waben with name or honig
        for (i, wabe) in self.waben.iter().enumerate() {
            if !wabe.name.is_empty() || wabe.honig != 0 {
                println!("Wabe {i} ({}) hat {} Honig", wabe.name, wabe.honig);
            }
        }
    }
}