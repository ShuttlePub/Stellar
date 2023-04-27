use std::collections::HashMap;
use destructure::Destructure;
use serde::{Serialize, Deserialize};

use crate::KernelError;

#[deprecated]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Scopes(Vec<ScopedObject>);

impl Scopes {
    pub fn add(&mut self, object: impl Into<ScopedObject>) {
        self.0.push(object.into());
    }
}

impl AsRef<[ScopedObject]> for Scopes {
    fn as_ref(&self) -> &[ScopedObject] {
        &self.0
    }
}

impl From<Scopes> for Vec<ScopedObject> {
    fn from(value: Scopes) -> Self {
        value.0
    }
}

impl From<Scopes> for Vec<(String, String)> {
    fn from(value: Scopes) -> Self {
        value.0.into_iter()
            .map(|object| object.into())
            .collect()
    }
}

impl From<Scopes> for Vec<Method> {
    fn from(value: Scopes) -> Self {
        value.0.into_iter()
            .map(|object| object.method)
            .collect()
    }
}

#[deprecated]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize, Destructure)]
pub struct ScopedObject {
    method: Method,
    description: MethodDescription
}

impl ScopedObject {
    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn description(&self) -> &MethodDescription {
        &self.description
    }
}

impl From<ScopedObject> for (String, String) {
    fn from(value: ScopedObject) -> Self {
        (value.method.into(), value.description.into())
    }
}

impl From<Method> for ScopedObject {
    fn from(value: Method) -> Self {
        Self {
            method: value,
            description: MethodDescription::default()
        }
    }
}

impl From<(Method, MethodDescription)> for ScopedObject {
    fn from(value: (Method, MethodDescription)) -> Self {
        Self { method: value.0, description: value.1 }
    }
}

#[deprecated]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Method(String, String);

impl Method {
    pub fn new(
        resource: impl Into<String>,
        method: impl Into<String>
    ) -> Self {
        Self(resource.into(), method.into())
    }

    pub fn resource(&self) -> &str {
        &self.0
    }

    pub fn method(&self) -> &str {
        &self.1
    }
}

impl From<Method> for String {
    fn from(value: Method) -> Self {
        format!("{}:{}", value.0, value.1)
    }
}

impl TryFrom<String> for Method {
    type Error = KernelError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let separated = value.split(':')
            .collect::<Vec<_>>();
        if separated.len() <= 1 || separated.len() > 2 {
            return Err(KernelError::InvalidValue {
                method: "`Method` try_from String",
                value: value.to_string()
            });
        }
        Ok(Self::new(separated[0], separated[1]))
    }
}

impl Serialize for Method {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where S: serde::Serializer {
        let scoped = format!("{}:{}", self.0, self.1);
        serializer.serialize_str(&scoped)
    }
}

impl<'de> Deserialize<'de> for Method {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
      where D: serde::Deserializer<'de> {
        let raw: &str = Deserialize::deserialize(deserializer)?;
        let scoped = raw.split(':')
            .fuse()
            .filter(|emptiness| !emptiness.is_empty())
            .collect::<Vec<_>>();
        Ok(Self::new(scoped[0], scoped[1]))
    }
}

#[deprecated]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct MethodDescription(String);

impl MethodDescription {
    pub fn new(desc: impl Into<String>) -> Self {
        Self(desc.into())
    }
}

impl From<MethodDescription> for String {
    fn from(value: MethodDescription) -> Self {
        value.0
    }
}

impl AsRef<str> for MethodDescription {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use super::{Method, MethodDescription, Scopes};

    #[test]
    fn new_scope() -> anyhow::Result<()> {
        let mut scopes = Scopes::default();
        scopes.add(Method::new("stellar", "read"));
        scopes.add((
            Method::new("test", "read"),
            MethodDescription::new("test client read")
        ));
        Ok(())
    }
}






#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct _Scopes(HashMap<_ScopeMethod, _ScopeDescription>);

impl _Scopes {
    pub fn new(values: impl Into<Vec<(_ScopeMethod, _ScopeDescription)>>) -> Self {
        Self(HashMap::from_iter(values.into().into_iter()))
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct _ScopeMethod(String);

impl _ScopeMethod {
    pub fn new(method: impl Into<String>) -> Self {
        Self(method.into())
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct _ScopeDescription(Option<String>);

impl _ScopeDescription {
    pub fn new<S: Into<String>, O: Into<Option<S>>>(desc: O) -> Self {
        Self(desc.into().map(Into::into))
    }
}