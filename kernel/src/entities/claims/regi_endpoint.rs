use rand::distributions::{Alphanumeric, Distribution};
use serde::{Deserialize, Serialize};
use url::Url;
use crate::BASE_URL;

#[derive(Debug, Clone, Hash, Deserialize, Serialize)]
pub struct RegistrationEndPoint(String);

impl RegistrationEndPoint {
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self(endpoint.into())
    }
}

impl Default for RegistrationEndPoint {
    fn default() -> Self {
        Self::new(Alphanumeric.sample_iter(&mut rand::thread_rng())
            .take(32)
            .map(char::from)
            .collect::<String>())
    }
}

impl From<RegistrationEndPoint> for Url {
    fn from(value: RegistrationEndPoint) -> Self {
        let mut url = BASE_URL.clone();
        url.set_path(&format!("client/{}", value.0));
        url
    }
}

impl From<RegistrationEndPoint> for String {
    fn from(value: RegistrationEndPoint) -> Self {
        value.0
    }
}

impl AsRef<str> for RegistrationEndPoint {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use url::Url;
    use crate::entities::RegistrationEndPoint;

    #[ignore ="Unit testing is not possible because of the use of environment variables."]
    #[test]
    fn endpoint_test() -> anyhow::Result<()> {
        let endpoint = RegistrationEndPoint::default();
        println!("{:?}", endpoint.as_ref());
        println!("{:?}", Url::from(endpoint).as_ref());
        Ok(())
    }
}