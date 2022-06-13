use crate::Name;

pub struct Winners {
    names: Vec<Name>,
}

impl Default for Winners {
    fn default() -> Self {
        Winners { names: vec![] }
    }
}

impl Winners {
    pub fn new(name: Vec<Name>) -> Self {
        Winners {
            names: name.clone(),
        }
    }

    pub fn names(&self) -> Vec<Name> {
        self.names.clone()
    }
}
