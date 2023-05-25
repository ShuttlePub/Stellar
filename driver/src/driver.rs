use std::time::Duration;
use deadpool_redis::{Pool as RedisPool, Config};
use lettre::{transport::smtp::authentication::Credentials, AsyncSmtpTransport};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use crate::config;

use crate::DriverError;

pub struct DataBaseDriver;

impl DataBaseDriver {
    pub async fn setup_postgres() -> Result<Pool<Postgres>, DriverError> {
        let url = Self::pg_env_setup();

        let pool = PgPoolOptions::new()
            .acquire_timeout(Duration::from_millis(5000))
            .max_connections(8)
            .connect(&url)
            .await?;

        sqlx::migrate!("../migrations")
            .run(&pool)
            .await?;

        config::initialize(pool.clone()).await?;

        Ok(pool)
    }

    fn pg_env_setup() -> String {
        dotenvy::dotenv().ok();
        dotenvy::var("PG_DATABASE_URL")
            .expect("`PG_DATABASE_URL` does not set! This value required.")
    }

    pub async fn setup_redis() -> Result<RedisPool, DriverError> {
        let url = Self::redis_env_setup();
        let cfg = Config::from_url(url);
        let pool = cfg.create_pool(Some(deadpool_redis::Runtime::Tokio1))?;
        Ok(pool)
    }

    fn redis_env_setup() -> String {
        dotenvy::dotenv().ok();
        dotenvy::var("REDIS_DATABASE_URL")
            .expect("`REDIS_DATABASE_URL` does not set! This value required.")
    }
}

pub struct SmtpDriver;

impl SmtpDriver {
    pub fn setup_lettre() -> Result<lettre::AsyncSmtpTransport<lettre::Tokio1Executor>, DriverError> {
        let (relay, address, pass) = Self::lettre_env_setup();
        let cred = Credentials::new(address, pass);
        let mailer = AsyncSmtpTransport::<lettre::Tokio1Executor>::relay(&relay)?
            .credentials(cred)
            .build();

        Ok(mailer)
    }

    fn lettre_env_setup() -> (String, String, String) {
        dotenvy::dotenv().ok();
        dotenvy::from_filename(".env.private").ok();
        let relay = dotenvy::var("RELAY_SERVER_URL")
            .expect("`RELAY_SERVER_URL` does not set! This value required.");
        let cred_address = dotenvy::var("SMTP_CREDENTIAL_ADDRESS")
            .expect("`SMTP_CREDENTIAL_ADDRESS` does not set! This value required.");
        let cred_pass = dotenvy::var("SMTP_CREDENTIAL_PASSWORD")
            .expect("`SMTP_CREDENTIAL_PASSWORD` does not set! This value required.");
        (relay, cred_address, cred_pass)
    }
}