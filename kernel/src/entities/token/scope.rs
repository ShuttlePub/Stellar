use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
pub struct Scope(Vec<String>);

impl Scope {
    pub fn new(scoped: impl Into<Vec<String>>) -> Self {
        Self(scoped.into())
    }
}

impl From<Scope> for Vec<String> {
    fn from(origin: Scope) -> Self {
        origin.0
    }
}

impl AsRef<[String]> for Scope {
    fn as_ref(&self) -> &[String] {
        &self.0
    }
}

// RFC7662 2.2. Introspection Response `scope`
impl Serialize for Scope {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where S: serde::Serializer {
        let scoped = self.0
            .join(" ");
        serializer.serialize_str(&scoped)
    }
}

// RFC7662 2.2. Introspection Response `scope`
impl<'de> Deserialize<'de> for Scope {
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