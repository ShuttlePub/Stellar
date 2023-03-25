use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct Active(bool);

impl From<Active> for bool {
    fn from(origin: Active) -> Self {
        origin.0
    }
}

impl AsRef<bool> for Active {
    fn as_ref(&self) -> &bool {
        &self.0
    }
}