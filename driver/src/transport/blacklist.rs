use crate::DriverError;
use kernel::interfaces::transport::BlackListTransporter;
use kernel::KernelError;
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashSet;

pub struct BlacklistRepository {
    client: Client,
}

impl BlacklistRepository {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

static BLACKLIST_REPO: Lazy<String> = Lazy::new(|| {
    dotenvy::var("BLACKLIST_REPO")
        .map(Into::into)
        .expect("`BLACKLIST_REPO` not set! This value require.")
});

#[derive(Debug, Clone, Deserialize)]
pub struct BlackList {
    blacklist: HashSet<String>,
}

#[async_trait::async_trait]
impl BlackListTransporter for BlacklistRepository {
    async fn pull(&self) -> Result<HashSet<String>, KernelError> {
        TlsRequestInternal::request(&self.client).await
    }
}

pub(in crate::transport) struct TlsRequestInternal;

impl TlsRequestInternal {
    async fn request(client: &Client) -> Result<HashSet<String>, KernelError> {
        let bl = client
            .get(&*BLACKLIST_REPO)
            .send()
            .await
            .map_err(DriverError::from)?
            .json::<BlackList>()
            .await
            .map_err(DriverError::from)?;

        Ok(bl.blacklist)
    }
}

#[cfg(test)]
mod tests {
    use crate::transport::blacklist::TlsRequestInternal;
    use reqwest::Client;

    #[ignore = "It depends on `reqwest` and does not work as is."]
    #[tokio::test]
    async fn test_request() -> anyhow::Result<()> {
        let client = Client::new();
        let list = TlsRequestInternal::request(&client).await?;

        println!("{:?}", list);
        Ok(())
    }
}
