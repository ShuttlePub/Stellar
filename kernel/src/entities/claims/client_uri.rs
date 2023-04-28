use serde::{Deserialize, Serialize};
use url::Url;
use crate::KernelError;

#[derive(Debug, Clone, Hash, Deserialize, Serialize)]
pub struct ClientUri(Url);

impl ClientUri {
    pub fn new(uri: impl AsRef<str>) -> Result<Self, KernelError> {
        let uri = uri.as_ref();
        Ok(Self(Url::parse(uri)
            .map_err(|e| KernelError::InvalidValue {
                method: "ClientUri init",
                value: format!("value: {:?}, serde: {:?}", uri, e),
            })?))
    }
}

impl From<ClientUri> for String {
    fn from(value: ClientUri) -> Self {
        value.0.into()
    }
}

impl AsRef<str> for ClientUri {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[cfg(test)]
mod test {
    use super::ClientUri;

    #[test]
    fn test() -> anyhow::Result<()> {
        let valid_url = "http://example.com";
        let invalid_url = "http;//example,com";

        let a = ClientUri::new(valid_url);
        assert!(a.is_ok());
        let a = ClientUri::new(invalid_url);
        assert!(a.is_err());
        // test case
        #[allow(clippy::unnecessary_to_owned)]
        let a = ClientUri::new(valid_url.to_string());
        assert!(a.is_ok());
        // test case
        #[allow(clippy::unnecessary_to_owned)]
        let a = ClientUri::new(invalid_url.to_string());
        assert!(a.is_err());
        Ok(())
    }
}