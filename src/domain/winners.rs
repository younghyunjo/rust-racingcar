use crate::Name;

pub struct Winners {
    names: Vec<Name>,
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
