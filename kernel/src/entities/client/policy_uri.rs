use serde::{Deserialize, Serialize};
use url::Url;
use crate::KernelError;

#[derive(Debug, Clone, Hash, Deserialize, Serialize)]
pub struct PolicyUri(Url);

impl PolicyUri {
    pub fn new(uri: impl AsRef<str>) -> Result<Self, KernelError> {
        let uri = uri.as_ref();
        Ok(Self(Url::parse(uri)
            .map_err(|e| KernelError::InvalidValue {
                method: "PolicyUri init",
                value: format!("value: {:?}, serde: {:?}", uri, e),
            })?))
    }
}

impl From<PolicyUri> for String {
    fn from(origin: PolicyUri) -> Self {
        origin.0.into()
    }
}

impl AsRef<str> for PolicyUri {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}