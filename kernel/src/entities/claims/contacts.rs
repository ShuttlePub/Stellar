use std::collections::HashSet;
use serde::{Deserialize, Serialize};

use super::Address;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Contacts(HashSet<Address>);

impl Contacts {
    pub fn new(adr: impl Into<Vec<Address>>) -> Self {
        Self(adr.into().into_iter().collect())
    }
    
    pub fn as_ref_vec(&self) -> Vec<&str> {
        self.0.iter().map(AsRef::as_ref).collect()
    }
}

impl From<Contacts> for Vec<Address> {
    fn from(values: Contacts) -> Self {
        values.into_iter().collect()
    }
}

impl IntoIterator for Contacts {
    type Item = Address;
    type IntoIter = std::collections::hash_set::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<Address> for Contacts {
    fn from_iter<T: IntoIterator<Item=Address>>(iter: T) -> Self {
        let v = iter.into_iter().collect::<Vec<Address>>();
        Self::new(v)
    }
}

impl From<Contacts> for HashSet<Address> {
    fn from(value: Contacts) -> Self {
        value.0
    }
}