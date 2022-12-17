use crate::Bienenstock;

pub trait Aufgabe {
    fn execute(&self, bienenstock: &Bienenstock);
    fn to_string(&self) -> Option<String>;
}

pub struct Leerlauf {}

impl Aufgabe for Leerlauf {
    fn execute(&self, _bienenstock: &Bienenstock) {}
    fn to_string(&self) -> Option<String> { None }
}

pub struct Tanzen {
    pub wabe_index: usize,
    pub name: bool
}

impl Aufgabe for Tanzen {
    fn execute(&self, bienenstock: &Bienenstock) {
        bienenstock.print(self.wabe_index, self.name);
    }

    fn to_string(&self) -> Option<String> {
        if self.name {
            Some(format!("tanzt den Namen von Wabe {}", self.wabe_index))
        } else {
            Some(format!("tanzt den Honig von Wabe {}", self.wabe_index))
        }
    }
}