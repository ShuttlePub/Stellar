use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct Subject(String);

impl Subject {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl From<Subject> for String {
    fn from(origin: Subject) -> Self {
        origin.0
    }
}

impl AsRef<str> for Subject {
    fn as_ref(&self) -> &str {
        &self.0
    }
}