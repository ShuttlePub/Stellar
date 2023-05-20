use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct State(String);

impl State {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl PartialEq<String> for State {
    fn eq(&self, other: &String) -> bool {
        self.0.eq(other)
    }
}

impl From<State> for String {
    fn from(value: State) -> Self { 
        value.0 
    }
}

impl AsRef<str> for State {
    fn as_ref(&self) -> &str {
       &self.0 
    }
}