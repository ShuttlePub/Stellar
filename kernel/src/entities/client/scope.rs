use crate::KernelError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use try_ref::TryAsRef;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Scopes(HashMap<ScopeMethod, ScopeDescription>);

impl Scopes {
    pub fn new(values: impl Into<Vec<(ScopeMethod, ScopeDescription)>>) -> Self {
        Self(HashMap::from_iter(values.into().into_iter()))
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ScopeMethod, &ScopeDescription)> {
        self.0.iter()
    }
}

impl From<Scopes> for HashMap<String, Option<String>> {
    fn from(value: Scopes) -> Self {
        value
            .into_iter()
            .map(|(method, desc)| (method.into(), desc.into()))
            .collect::<HashMap<String, Option<String>>>()
    }
}

impl From<Scopes> for Vec<(ScopeMethod, ScopeDescription)> {
    fn from(values: Scopes) -> Self {
        values.into_iter().collect()
    }
}

impl AsRef<HashMap<ScopeMethod, ScopeDescription>> for Scopes {
    fn as_ref(&self) -> &HashMap<ScopeMethod, ScopeDescription> {
        &self.0
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
    fn from_iter<T: IntoIterator<Item = (ScopeMethod, ScopeDescription)>>(iter: T) -> Self {
        let v = iter
            .into_iter()
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

impl From<ScopeDescription> for Option<String> {
    fn from(value: ScopeDescription) -> Self {
        value.0
    }
}

impl TryAsRef<str> for ScopeDescription {
    type Error = KernelError;
    fn try_as_ref(&self) -> Result<&str, Self::Error> {
        match self.0 {
            Some(ref inner) => Ok(inner),
            None => Err(KernelError::InvalidValue {
                method: "try_as_ref",
                value: "scope description".to_string(),
            }),
        }
    }
}

impl AsRef<Option<String>> for ScopeDescription {
    fn as_ref(&self) -> &Option<String> {
        &self.0
    }
}
