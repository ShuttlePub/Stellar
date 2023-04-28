use std::collections::HashSet;
use serde::{Deserialize, Serialize};

use super::Address;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Contacts(HashSet<Address>);

impl Contacts {
    pub fn new(adr: impl Into<Vec<Address>>) -> Self {
        Self(adr.into().into_iter().collect())
    }
}

impl IntoIterator for Contacts {
    type Item = Address;
    type IntoIter = std::collections::hash_set::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl From<Contacts> for HashSet<Address> {
    fn from(value: Contacts) -> Self {
        value.0
    }
}

impl AsRef<HashSet<Address>> for Contacts {
    fn as_ref(&self) -> &HashSet<Address> {
        &self.0
    }
}