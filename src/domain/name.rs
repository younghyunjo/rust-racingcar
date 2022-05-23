use std::fmt::{Debug, Display, Formatter};
use thiserror::Error;

#[derive(Error, Debug)]
#[error("{message:}")]
pub struct NameError {
    pub message: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Name {
    name: String,
}

const MAX_NAME_LEN: usize = 5;

impl Name {
    pub fn new(name: &str) -> Result<Self, NameError> {
        if name.len() > MAX_NAME_LEN {
            return Err(NameError {
                message: "name is too long".to_string(),
            });
        }
        Ok(Name {
            name: name.to_string(),
        })
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::name::Name;

    #[test]
    fn given_name_when_new_then_ok() {
        //given
        let name = "name";

        //when
        let ret = Name::new(name);

        //then
        assert_eq!(ret.is_ok(), true);
    }

    #[test]
    fn given_long_name_when_new_then_err() {
        //given
        let name = "long_name";

        //when
        let ret = Name::new(name);

        //thenx
        assert_eq!(ret.is_err(), true);
    }
}
