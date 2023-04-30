use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Scopes(HashMap<ScopeMethod, ScopeDescription>);

impl Scopes {
    pub fn new(values: impl Into<Vec<(ScopeMethod, ScopeDescription)>>) -> Self {
        Self(HashMap::from_iter(values.into().into_iter()))
    }
}

impl From<Scopes> for Vec<(ScopeMethod, ScopeDescription)> {
    fn from(values: Scopes) -> Self {
        values.into_iter().collect()
    }
}

impl IntoIterator for Scopes {
    type Item = (ScopeMethod, ScopeDescription);
    type IntoIter = std::collections::hash_map::IntoIter<ScopeMethod, ScopeDescription>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<(ScopeMethod, ScopeDescription)> for Scopes {
    fn from_iter<T: IntoIterator<Item=(ScopeMethod, ScopeDescription)>>(iter: T) -> Self {
        let v = iter.into_iter()
            .collect::<Vec<(ScopeMethod, ScopeDescription)>>();
        Self::new(v)
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct ScopeMethod(String);

impl ScopeMethod {
    pub fn new(method: impl Into<String>) -> Self {
        Self(method.into())
    }
}

impl From<ScopeMethod> for String {
    fn from(value: ScopeMethod) -> Self {
        value.0
    }
}

impl AsRef<str> for ScopeMethod {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct ScopeDescription(Option<String>);

impl ScopeDescription {
    pub fn new(desc: impl Into<Option<String>>) -> Self {
        Self(desc.into().map(Into::into))
    }
}

impl From<ScopeDescription> for Option<String>  {
    fn from(value: ScopeDescription) -> Self {
        value.0
    }
}

impl AsRef<Option<String>> for ScopeDescription {
    fn as_ref(&self) -> &Option<String> {
        &self.0
    }
}