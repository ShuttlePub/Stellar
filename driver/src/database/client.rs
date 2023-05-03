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

        if let Err(r) = PgClientInternal::insert(client, &mut transaction).await {
            transaction.rollback().await
                .map_err(DriverError::SqlX)?;
            return Err(KernelError::Driver(anyhow::Error::msg("failed transaction in client registration.")))
        }

        Ok(())
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
              $4::TEP_AM,
              $5::GRANT_TYPE[],
              $6::RESPONSE_TYPE[],
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

        // language=SQL
        sqlx::query(r#"
            INSERT INTO client_scopes(
              client_id,
              method,
              description
            )
            SELECT
              $1,
              * FROM UNNEST(
                  $2::VARCHAR[],
                  $3::VARCHAR[]
                )
        "#)
        .bind(client.id().id())
        .bind(client.scopes().iter()
            .map(|(method, _)| method)
            .map(AsRef::as_ref)
            .collect::<Vec<_>>())
        .bind(client.scopes().iter()
            .map(|(_, desc)| desc)
            .map(TryAsRef::<str>::try_as_ref)
            .map(Result::ok)
            .collect::<Vec<_>>())
        .execute(&mut *con)
        .await?;

        // language=SQL
        sqlx::query(r#"
            INSERT INTO client_configuration_policy(
              client_id, endpoint, token
            ) VALUES (
              $1, $2, $3
            )
        "#)
        .bind(client.id().id())
        .bind(client.conf_endpoint().as_ref())
        .bind(client.conf_token().as_ref())
        .execute(&mut *con)
        .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use sqlx::{Pool, Postgres};
    use sqlx::postgres::PgPoolOptions;
    use kernel::entities::{Account, Address, Client, ClientId, ClientSecret, ClientTypes, Contacts, GrantType, RedirectUri, RedirectUris, RegistrationAccessToken, RegistrationEndPoint, ResponseType, ScopeDescription, ScopeMethod, Scopes, TokenEndPointAuthMethod, UserId};
    use kernel::external::{OffsetDateTime, Uuid};
    use crate::database::account::PgAccountInternal;
    use crate::database::client::PgClientInternal;

    async fn test_pool() -> anyhow::Result<Pool<Postgres>> {
        dotenvy::dotenv().ok();

        let url = dotenvy::var("PG_DATABASE_URL")
            .expect("`DATABASE_URL` is not set. This is a required environment variable.");
        let pool = PgPoolOptions::new()
            .max_connections(4)
            .idle_timeout(Duration::new(5, 0))
            .connect(&url)
            .await?;

        Ok(pool)
    }

    #[ignore = "It depends on Postgres and does not work as is."]
    #[tokio::test]
    async fn pg_insert() -> anyhow::Result<()> {
        let pool = test_pool().await?;

        let mut transaction = pool.begin().await?;

        let client_id = ClientId::new_at_now(Uuid::new_v4());
        let client_name = "Test Client";
        let client_uri = "https://test.client.example.com/";
        let client_desc = "TEST CLIENT!";
        let client_type = ClientTypes::new(ClientSecret::default());
        let logo_uri = "https://test.client.example.com/logo";
        let tos_uri = "https://test.client.example.com/terms";
        let owner_id = UserId::default();
        let policy_uri = "https://test.client.example.com/policy";
        let auth_method = TokenEndPointAuthMethod::ClientSecretPost;
        let grant_types = vec![GrantType::AuthorizationCode];
        let response_types = vec![ResponseType::Code];
        let redirect_uris = vec![
            "https://test.client.example.com/callback",
            "https://test.client.example.com/callback2"
        ].into_iter()
            .map(RedirectUri::new)
            .collect::<RedirectUris>();
        let scopes = vec![
            ("read", Some("base user data read")),
            ("write", Some("base user data write")),
            ("phantom", None)
        ].into_iter()
            .map(|(method, desc)| (ScopeMethod::new(method), ScopeDescription::new(desc.map(ToOwned::to_owned))))
            .collect::<Scopes>();
        let contacts = vec!["test.user@client.com"]
            .into_iter()
            .map(Address::new)
            .collect::<Contacts>();
        let regi_token = RegistrationAccessToken::default();
        let regi_endpoint = RegistrationEndPoint::default();

        let dummy_account = Account::new(
            owner_id,
            "test.user@example.com",
            "test user",
            "test0000pAssw0rd",
            OffsetDateTime::now_utc(),
            OffsetDateTime::now_utc(),
            OffsetDateTime::now_utc()
        )?;

        let reg = Client::new(
            client_id,
            client_name,
            client_uri,
            client_desc,
            client_type,
            logo_uri,
            tos_uri,
            owner_id,
            policy_uri,
            auth_method,
            grant_types,
            response_types,
            redirect_uris,
            scopes,
            contacts,
            None,
            regi_token,
            regi_endpoint
        )?;

        PgAccountInternal::create(&dummy_account, &mut transaction).await?;
        PgClientInternal::insert(&reg, &mut transaction).await?;

        transaction.rollback().await?;

        Ok(())
    }
}