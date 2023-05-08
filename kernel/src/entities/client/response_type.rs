use std::collections::HashSet;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::KernelError;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResponseTypes(HashSet<ResponseType>);

impl ResponseTypes {
    pub fn new(types: impl Into<Vec<ResponseType>>) -> Self {
        Self(types.into().into_iter().collect())
    }

    pub fn iter(&self) -> impl Iterator<Item = &ResponseType> {
        self.0.iter()
    }
}

impl From<ResponseTypes> for Vec<ResponseType> {
    fn from(values: ResponseTypes) -> Self {
        values.0.into_iter().collect()
    }
}

impl IntoIterator for ResponseTypes {
    type Item = ResponseType;
    type IntoIter = std::collections::hash_set::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<ResponseType> for ResponseTypes {
    fn from_iter<T: IntoIterator<Item=ResponseType>>(iter: T) -> Self {
        let v = iter.into_iter()
            .collect::<Vec<ResponseType>>();
        Self::new(v)
    }
}

impl From<ResponseTypes> for HashSet<ResponseType> {
    fn from(value: ResponseTypes) -> Self {
        value.0
    }
}

impl AsRef<HashSet<ResponseType>> for ResponseTypes {
    fn as_ref(&self) -> &HashSet<ResponseType> {
        &self.0
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub enum ResponseType {
    Code,
    Token
}

impl TryFrom<String> for ResponseType {
    type Error = KernelError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        ResponseType::from_str(value.as_str())
    }
}

impl FromStr for ResponseType {
    type Err = KernelError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "code" => Self::Code,
            "token" => Self::Token,
            _ => return Err(KernelError::InvalidValue {
                method: "from_str",
                value: s.to_string(),
            })
        })
    }
}

impl AsRef<str> for ResponseType {
    fn as_ref(&self) -> &str {
        match self {
            ResponseType::Code => "code",
            ResponseType::Token => "token"
        }
    }
}