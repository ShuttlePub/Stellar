use serde::{Deserialize, Serialize};
use kernel::prelude::entities::{Account, Client, ClientId, UserId};
use super::{AdminUser, StellarClient};
use crate::DriverError;


#[derive(Serialize, Deserialize)]
pub struct GenIds {
    pub admin_id: UserId,
    pub stellar_id: ClientId
}

impl GenIds {
    pub fn new(admin_id: UserId, stellar_id: ClientId) -> Self {
        Self { admin_id, stellar_id }
    }
}


#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Config {
    pub admin: Admin,
    pub stellar: Stellar
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Admin {
    pub address: String,
    pub name: String,
    pub pass: String,
    pub pass_hashed: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Stellar {
    pub contacts: Vec<String>,
    pub client_uri: String,
    pub logo_uri: String,
    pub tos_uri: String,
    pub policy_uri: String,
    pub jwks: Option<String>,
    pub jwks_uri: Option<String>,
}

impl Default for Admin {
    fn default() -> Self {
        Self {
            address: "admin@example.com".into(),
            name: "administrator".into(),
            pass: "administrator".into(),
            pass_hashed: None
        }
    }
}

impl Default for Stellar {
    fn default() -> Self {
        Self {
            contacts: vec!["example.user@stellar.example.com".into()],
            client_uri: "https://stellar.example.com/".into(),
            logo_uri: "https://stellar.example.com/logo".into(),
            tos_uri: "https://stellar.example.com/terms".into(),
            policy_uri: "https://stellar.example.com/policy".into(),
            jwks: None,
            jwks_uri: Some("https://stellar.example.com/.well-known".into())
        }
    }
}

impl Config {
    pub fn formed(
        self,
        admin_id: UserId,
        stellar_id: ClientId
    ) -> Result<(Account, Client), DriverError> {
        let Config { admin, stellar } = self;

        let mut admin: AdminUser = admin.try_into()?;
        admin.user_id(admin_id);

        let mut stellar: StellarClient = stellar.try_into()?;
        stellar.client_id(stellar_id);
        stellar.owner(admin_id);

        let admin: Account = admin.try_into()?;
        let stellar: Client = stellar.try_into()?;

        Ok((admin, stellar))
    }

}


#[cfg(test)]
mod tests {
    use super::Config;

    fn load_config(config: impl AsRef<str>) -> anyhow::Result<Config> {
        let config = toml::from_str::<Config>(config.as_ref())?;
        Ok(config)
    }

    #[test]
    fn load_from_str() -> anyhow::Result<()> {
        // language=TOML
        let toml = r#"[admin]
address = "admin@example.com"
name = "administrator"
pass = "administrator"

[stellar]
contacts = ["admin@example.com"]
client_uri = "https://stellar.example.com/"
logo_uri = "https://stellar.example.com/logo"
tos_uri = "https://stellar.example.com/terms"
policy_uri = "https://stellar.example.com/policy"
jwks_uri = "https://stellar.example.com/.well-known"
"#;

        let config = load_config(toml)?;

        let ser = toml::to_string(&config)?;

        println!("{}", ser);
        assert_eq!(toml, ser);
        Ok(())
    }
}