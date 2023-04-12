use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
pub struct Scopes(Vec<String>);

impl Scopes {
    pub fn new(scoped: impl Into<Vec<String>>) -> Self {
        Self(scoped.into())
    }
}

impl From<Scopes> for Vec<String> {
    fn from(origin: Scopes) -> Self {
        origin.0
    }
}

impl AsRef<[String]> for Scopes {
    fn as_ref(&self) -> &[String] {
        &self.0
    }
}

// RFC7662 2.2. Introspection Response `scope`
impl Serialize for Scopes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where S: serde::Serializer {
        let scoped = self.0
            .join(" ");
        serializer.serialize_str(&scoped)
    }
}

// RFC7662 2.2. Introspection Response `scope`
impl<'de> Deserialize<'de> for Scopes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
      where D: serde::Deserializer<'de> {
        let raw: &str = Deserialize::deserialize(deserializer)?;
        let scoped = raw.split(' ')
            .fuse()
            .filter(|emptiness| !emptiness.is_empty())
            .map(Into::into)
            .collect::<Vec<String>>();
        Ok(Self(scoped))
    }
}