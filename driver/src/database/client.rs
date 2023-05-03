use futures::{StreamExt, TryStreamExt};
use futures::future::ready;
use sqlx::{PgConnection, Pool, Postgres};
use kernel::entities::{Client, ClientId};
use kernel::external::JsonWebKey;
use kernel::KernelError;
use kernel::repository::ClientRegistry;
use try_ref::TryAsRef;
use crate::DriverError;

pub struct ClientDataBase {
    pool: Pool<Postgres>
}

impl ClientDataBase {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl ClientRegistry for ClientDataBase {
    async fn register(&self, client: &Client) -> Result<(), KernelError> {
        let mut transaction = self.pool.begin().await
            .map_err(DriverError::SqlX)?;
        PgClientInternal::insert(client, &mut transaction).await?;


        todo!()
    }

    async fn delete(&self, id: &ClientId) -> Result<(), KernelError> {
        todo!()
    }

    async fn update(&self, client: &Client) -> Result<(), KernelError> {
        todo!()
    }

    async fn find_by_id(&self, id: &ClientId) -> Result<Option<Client>, KernelError> {
        todo!()
    }
}

pub(in crate::database) struct PgClientInternal;

impl PgClientInternal {
    async fn insert(client: &Client, con: &mut PgConnection) -> Result<(), DriverError> {
        // language=SQL
        sqlx::query(r#"
            INSERT INTO clients(
              client_id,
              client_id_iat,
              client_name
            )
            VALUES (
              $1,
              $2,
              $3
            )
        "#)
        .bind(client.id().id())
        .bind(client.id().issued_at())
        .bind(client.name().as_ref())
        .execute(&mut *con)
        .await?;

        // language=SQL
        sqlx::query(r#"
            INSERT INTO client_metadata(
              owner,
              client_id,
              client_uri,
              logo_uri,
              contact,
              tos_uri,
              policy_uri
            ) VALUES (
              $1,
              $2,
              $3,
              $4,
              $5,
              $6,
              $7
            )
        "#)
        .bind(client.owner().as_ref())
        .bind(client.id().id())
        .bind(client.client_uri().as_ref())
        .bind(client.logo_uri().as_ref())
        .bind(client.contacts().as_ref_vec())
        .bind(client.tos_uri().as_ref())
        .bind(client.policy_uri().as_ref())
        .execute(&mut *con).await?;

        // language=SQL
        sqlx::query(r#"
            INSERT INTO client_cert(
              client_id,
              client_secret,
              client_secret_exp,
              auth_method,
              grant_types,
              response_types,
              jwks_uri,
              jwks
            ) VALUES (
              $1,
              $2,
              $3,
              $4,
              $5,
              $6,
              $7,
              $8
            )
        "#)
        .bind(client.id().id())
        .bind(client.types().as_ref()
            .map(|secret| secret.secret()))
        .bind(client.types().as_ref()
            .map(|secret| secret.expires_at()))
        .bind(client.auth_method().as_ref())
        .bind(client.grant_types().iter()
            .map(AsRef::as_ref)
            .collect::<Vec<_>>())
        .bind(client.response_types().iter()
            .map(AsRef::as_ref)
            .collect::<Vec<_>>())
        .bind(client.jwks().as_ref()
            .filter(|key| key.is_uri())
            .map(TryAsRef::<str>::try_as_ref)
            .transpose()?)
        .bind(client.jwks().as_ref()
            .filter(|key| !key.is_uri())
            .map(TryAsRef::<JsonWebKey>::try_as_ref)
            .transpose()?
            .map(serde_json::to_value)
            .transpose()
            .map_err(|e| KernelError::External(anyhow::Error::new(e)))?)
        .execute(&mut *con)
        .await?;

        // language=SQL
        sqlx::query(r#"
            INSERT INTO client_redirect_uris(
              client_id, uri
            ) VALUES (
              $1, $2
            )
        "#)
        .bind(client.id().id())
        .bind(client.redirect_uris().iter()
            .map(AsRef::as_ref)
            .collect::<Vec<_>>())
        .execute(&mut *con)
        .await?;


        let st = futures::stream::iter(client.scopes().iter());


        todo!()
    }
}