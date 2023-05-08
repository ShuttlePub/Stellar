use serde::{Deserialize, Serialize};
use url::Url;
use crate::KernelError;

#[derive(Debug, Clone, Hash, Deserialize, Serialize)]
pub struct LogoUri(Url);

impl LogoUri {
    pub fn new(uri: impl AsRef<str>) -> Result<Self, KernelError> {
        let uri = uri.as_ref();
        Ok(Self(Url::parse(uri)
            .map_err(|e| KernelError::InvalidValue {
                method: "LogoUri init",
                value: format!("value: {:?}, serde: {:?}", uri, e),
            })?))
    }
}

impl From<LogoUri> for String {
    fn from(value: LogoUri) -> Self {
        value.0.into()
    }
}

impl AsRef<str> for LogoUri {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}