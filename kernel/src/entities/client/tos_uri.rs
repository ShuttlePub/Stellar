use crate::KernelError;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Hash, Deserialize, Serialize)]
pub struct TermsUri(Url);

impl TermsUri {
    pub fn new(uri: impl AsRef<str>) -> Result<Self, KernelError> {
        let uri = uri.as_ref();
        Ok(Self(Url::parse(uri).map_err(|e| {
            KernelError::InvalidValue {
                method: "TermsUri init",
                value: format!("value: {:?}, serde: {:?}", uri, e),
            }
        })?))
    }
}

impl From<TermsUri> for String {
    fn from(value: TermsUri) -> Self {
        value.0.into()
    }
}

impl AsRef<str> for TermsUri {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}
