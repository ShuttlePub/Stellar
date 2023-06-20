mod a_gen;
mod a_load;
mod model;
mod admin;
mod stellar;

use self::{
    a_gen::*,
    a_load::*,
    model::*,
    admin::*,
    stellar::*,
};

use sqlx::{Pool, Postgres};
use once_cell::sync::Lazy;
use kernel::interfaces::repository::{AccountRepository, ClientRegistry};
use crate::database::{AccountDataBase, ClientDataBase};
use crate::DriverError;

pub(in crate::config) mod constants {
    pub const CONFIG: &str = "stellar.toml";
    pub const GENNED: &str = "stellar.g.bin";
    pub const CACHED: &str = "stellar.c.bin";
}

static BASE: Lazy<String> = Lazy::new(|| {
    dotenvy::var("STELLAR_CONF_DIR")
        .unwrap_or("./".to_string())
});


pub async fn initialize(pool: Pool<Postgres>) -> Result<(), DriverError> {
    let account_database = AccountDataBase::new(pool.clone());
    let client_database = ClientDataBase::new(pool);

    match generate(&*BASE)? {
        Some((admin, stellar)) => {
            account_database.create(&admin).await?;
            client_database.register(&stellar).await?;
            Ok(())
        }
        None => {
            let Some((admin, stellar)) = load(&*BASE)? else {
                tracing::info!("No change in config. skipped this task.");
                return Ok(())
            };

            account_database.update(&admin).await?;
            client_database.update(&stellar).await?;
            Ok(())
        }
    }
}