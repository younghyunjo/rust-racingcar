use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use std::str::FromStr;
use thiserror::Error;

const MAX_NAME_LEN: usize = 5;

#[derive(Error, Debug)]
#[error("{message:}")]
pub struct NameError {
    pub message: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Name {
    name: String,
}

impl Deref for Name {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.name
    }
}

impl Default for Name {
    fn default() -> Self {
        Name {
            name: "".to_string(),
        }
    }
}

impl AsRef<String> for Name {
    fn as_ref(&self) -> &String {
        &self.name
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        self.name.as_str()
    }
}

impl FromStr for Name {
    type Err = NameError;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        Name::new(name.into())
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Name {
    pub fn new(name: String) -> Result<Self, NameError> {
        if name.len() > MAX_NAME_LEN {
            return Err(NameError {
                message: "name is too long".to_string(),
            });
        }
        Ok(Name { name })
    }

    pub fn name(&self) -> String {
        self.name.clone()
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
        let ret = Name::new(name.into());

        //then
        assert_eq!(ret.is_ok(), true);
    }

    #[test]
    fn given_long_name_when_new_then_err() {
        //given
        let name = "long_name";
        //when
        let ret = Name::new(name.into());

        //thenx
        assert_eq!(ret.is_err(), true);
    }

    #[test]
    fn given_sting_type_name_when_new_then_ok() {
        let name = String::from("name");

        let ret = Name::new(name);

        //then
        assert_eq!(ret.is_ok(), true);
    }
}
