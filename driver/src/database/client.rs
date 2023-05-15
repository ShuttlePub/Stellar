use std::collections::HashMap;
use sqlx::{PgConnection, Pool, Postgres};
use sqlx::types::Json;
use kernel::entities::{
    Address,
    Client,
    ClientId,
    ClientSecret,
    ClientTypes,
    GrantType,
    RedirectUri,
    ResponseType,
    ScopeDescription,
    ScopeMethod,
    TokenEndPointAuthMethod
};
use kernel::external::{JsonWebKey, OffsetDateTime, Uuid};
use kernel::KernelError;
use kernel::repository::ClientRegistry;
use try_ref::TryAsRef;
use merge_opt::merge_opt_i2;
use crate::DriverError;

#[derive(Clone)]
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
            return Err(KernelError::Driver(anyhow::Error::new(r)))
        }

        transaction.commit().await
            .map_err(DriverError::SqlX)?;

        Ok(())
    }

    async fn delete(&self, id: &ClientId) -> Result<(), KernelError> {
        let mut transaction = self.pool.begin().await
            .map_err(DriverError::SqlX)?;
        if let Err(r) = PgClientInternal::delete(id, &mut transaction).await {
            transaction.rollback().await
                .map_err(DriverError::SqlX)?;
            return Err(KernelError::Driver(anyhow::Error::new(r)))
        }

        transaction.commit().await
            .map_err(DriverError::SqlX)?;

        Ok(())
    }

    async fn update(&self, client: &Client) -> Result<(), KernelError> {
        let mut transaction = self.pool.begin().await
            .map_err(DriverError::SqlX)?;

        if let Err(r) = PgClientInternal::update(client, &mut transaction).await {
            transaction.rollback().await
                .map_err(DriverError::SqlX)?;
            return Err(KernelError::Driver(anyhow::Error::new(r)))
        }

        transaction.commit().await
            .map_err(DriverError::SqlX)?;

        Ok(())
    }

    async fn find_by_id(&self, id: &ClientId) -> Result<Option<Client>, KernelError> {
        let mut con = self.pool.acquire().await
            .map_err(DriverError::SqlX)?;
        let client = PgClientInternal::find_by_id(id, &mut con).await?;
        Ok(client)
    }
}

#[derive(sqlx::FromRow, Debug)]
struct ClientRow {
    client_id: Uuid,
    client_id_iat: OffsetDateTime,
    client_name: String,
    description: String,
    owner: Uuid,
    client_uri: String,
    logo_uri: String,
    tos_uri: String,
    policy_uri: String,
    contact: Vec<String>,
    client_secret: Option<String>,
    client_secret_exp: Option<OffsetDateTime>,
    auth_method: String,
    grant_types: Vec<String>,
    response_types: Vec<String>,
    jwks: Option<Json<JsonWebKey>>,
    jwks_uri: Option<String>,
    redirect_uris: Vec<String>,
    scope: Json<HashMap<String, Option<String>>>,
    registration_token: String,
    registration_endpoint: String
}

pub(in crate::database) struct PgClientInternal;

impl PgClientInternal {
    //noinspection DuplicatedCode
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
              description,
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
              $7,
              $8
            )
        "#)
        .bind(client.description().as_ref())
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
        .bind(client.types().try_as_ref().ok()
            .map(|secret| secret.secret()))
        .bind(client.types().try_as_ref().ok()
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
              scope
            )
            SELECT
              $1,
              $2
        "#)
        .bind(client.id().id())
        .bind(serde_json::to_value(client.scopes())
            .map_err(|e| KernelError::External(anyhow::Error::new(e)))?)
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

    async fn delete(id: &ClientId, con: &mut PgConnection) -> Result<(), DriverError> {
        // language=SQL
        sqlx::query(r#"
            DELETE FROM clients WHERE client_id = $1
        "#)
        .bind(id.id())
        .execute(&mut *con)
        .await?;
        Ok(())
    }

    //noinspection DuplicatedCode
    async fn update(client: &Client, con: &mut PgConnection) -> Result<(), DriverError> {
        // language=SQL
        sqlx::query(r#"
            UPDATE clients
              SET
                client_name = $1
            WHERE
              client_id = $2
        "#)
        .bind(client.name().as_ref())
        .bind(client.id().id())
        .execute(&mut *con)
        .await?;

        // language=SQL
        sqlx::query(r#"
            UPDATE client_metadata
              SET
                owner = $1,
                client_uri = $2,
                logo_uri = $3,
                contact = $4,
                tos_uri = $5,
                policy_uri = $6
            WHERE client_id = $7
        "#)
        .bind(client.owner().as_ref())
        .bind(client.client_uri().as_ref())
        .bind(client.logo_uri().as_ref())
        .bind(client.contacts().as_ref_vec())
        .bind(client.tos_uri().as_ref())
        .bind(client.policy_uri().as_ref())
        .bind(client.id().id())
        .execute(&mut *con)
        .await?;

        // language=SQL
        sqlx::query(r#"
            UPDATE client_cert
              SET
                client_secret = $1,
                client_secret_exp = $2,
                auth_method = $3::TEP_AM,
                grant_types = $4::GRANT_TYPE[],
                response_types = $5::RESPONSE_TYPE[],
                jwks_uri = $6,
                jwks = $7
            WHERE
              client_id = $8
        "#)
        .bind(client.types().try_as_ref().ok()
            .map(|secret| secret.secret()))
        .bind(client.types().try_as_ref().ok()
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
        .bind(client.id().id())
        .execute(&mut *con)
        .await?;

        // language=SQL
        sqlx::query(r#"
            UPDATE client_redirect_uris
              SET
                uri = $1
            WHERE
              client_id = $2
        "#)
        .bind(client.response_types().iter()
            .map(AsRef::as_ref)
            .collect::<Vec<_>>())
        .bind(client.id().id())
        .execute(&mut *con)
        .await?;

        // language=SQL
        sqlx::query(r#"
            UPDATE client_scopes
              SET
                scope = $1
            WHERE
              client_id = $2
        "#)
        .bind(serde_json::to_value(client.scopes())
            .map_err(|e| KernelError::External(anyhow::Error::new(e)))?)
        .bind(client.id().id())
        .execute(&mut *con)
        .await?;

        Ok(())
    }

    async fn find_by_id(id: &ClientId, con: &mut PgConnection) -> Result<Option<Client>, DriverError> {
        // Note: L406-409 See https://github.com/launchbadge/sqlx/issues/298
        // language=SQL
        let fetched = sqlx::query_as::<_, ClientRow>(r#"
            SELECT
              c.client_id,
              c.client_id_iat,
              c.client_name,
              cm.description,
              cm.owner,
              cm.client_uri,
              cm.logo_uri,
              cm.tos_uri,
              cm.policy_uri,
              cm.contact,
              cc.client_secret,
              cc.client_secret_exp,
              cc.auth_method::TEXT,
              cc.grant_types::TEXT[],
              cc.response_types::TEXT[],
              cc.jwks,
              cc.jwks_uri,
              cru.uri as redirect_uris,
              cs.scope,
              ccp.token as registration_token,
              ccp.endpoint as registration_endpoint
            FROM clients c
              LEFT JOIN client_metadata             cm  on c.client_id = cm.client_id
                   JOIN client_cert                 cc  on c.client_id = cc.client_id
                   JOIN client_scopes               cs  on c.client_id = cs.client_id
                   JOIN client_redirect_uris        cru on c.client_id = cru.client_id
                   JOIN client_configuration_policy ccp on c.client_id = ccp.client_id
            WHERE c.client_id = $1
        "#)
        .bind(id.id())
        .fetch_optional(&mut *con)
        .await?
            .map(|row| -> Result<Client, DriverError> {
                Ok(Client::new(
                    ClientId::new(
                        row.client_id,
                        row.client_id_iat
                    ),
                    row.client_name,
                    row.client_uri,
                    row.description,
                    ClientTypes::new(merge_opt_i2(
                        row.client_secret,
                        row.client_secret_exp,
                        |secret, exp| {
                        ClientSecret::new(secret?, exp?).into()
                    })),
                    row.logo_uri,
                    row.tos_uri,
                    row.owner,
                    row.policy_uri,
                    TokenEndPointAuthMethod::try_from(row.auth_method)?,
                    row.grant_types.into_iter()
                        .map(TryFrom::try_from)
                        .collect::<Result<Vec<GrantType>, KernelError>>()?,
                    row.response_types.into_iter()
                        .map(TryFrom::try_from)
                        .collect::<Result<Vec<ResponseType>, KernelError>>()?,
                    row.redirect_uris.into_iter()
                        .map(RedirectUri::new)
                        .collect::<Vec<_>>(),
                    row.scope.0.into_iter()
                        .map(|scope| (ScopeMethod::new(scope.0), ScopeDescription::new(scope.1)))
                        .collect::<Vec<_>>(),
                    row.contact.into_iter()
                        .map(Address::new)
                        .collect::<Vec<_>>(),
                    merge_opt_i2(
                        row.jwks,
                        row.jwks_uri, |jwk, uri| {
                            if let Some(jwk) = jwk {
                                return jwk.0.to_string().into()
                            }
                            uri?.into()
                        }),
                    row.registration_token,
                    row.registration_endpoint
                )?)
            })
            .transpose()?;
        Ok(fetched)
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};
    use sqlx::{PgConnection, Pool, Postgres};
    use sqlx::postgres::PgPoolOptions;
    use kernel::entities::{Account, Address, Client, ClientId, ClientSecret, ClientTypes, ClientUri, Contacts, GrantType, RedirectUri, RedirectUris, RegistrationAccessToken, RegistrationEndPoint, ResponseType, ScopeDescription, ScopeMethod, Scopes, TokenEndPointAuthMethod, UserId};
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

    async fn create_dummy_data(con: &mut PgConnection) -> anyhow::Result<Client> {
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

        let timer = Instant::now();

        PgAccountInternal::create(&dummy_account, con).await?;
        PgClientInternal::insert(&reg, con).await?;

        println!("account + client insert time: {}ms", timer.elapsed().as_millis());

        Ok(reg)
    }

    #[ignore = "It depends on Postgres and does not work as is."]
    #[tokio::test]
    async fn pg_insert() -> anyhow::Result<()> {
        let pool = test_pool().await?;

        let mut transaction = pool.begin().await?;

        create_dummy_data(&mut transaction).await?;

        transaction.rollback().await?;

        Ok(())
    }

    #[ignore = "It depends on Postgres and does not work as is."]
    #[tokio::test]
    async fn pg_delete() -> anyhow::Result<()> {
        let pool = test_pool().await?;

        let mut transaction = pool.begin().await?;

        let client = create_dummy_data(&mut transaction).await?;

        PgClientInternal::delete(client.id(), &mut transaction).await?;

        transaction.rollback().await?;

        Ok(())
    }

    #[ignore = "It depends on Postgres and does not work as is."]
    #[tokio::test]
    async fn pg_update() -> anyhow::Result<()> {
        let pool = test_pool().await?;

        let mut transaction = pool.begin().await?;

        let client = create_dummy_data(&mut transaction).await?;

        let mut client = client.into_destruct();

        client.uri = ClientUri::new("https://example.client.com/")?;

        let client = client.freeze();

        PgClientInternal::update(&client, &mut transaction).await?;

        transaction.rollback().await?;

        Ok(())
    }

    #[ignore = "It depends on Postgres and does not work as is."]
    #[tokio::test]
    async fn pg_select() -> anyhow::Result<()> {
        let pool = test_pool().await?;

        let mut transaction = pool.begin().await?;

        let client = create_dummy_data(&mut transaction).await?;
        let timer = Instant::now();

        let fetched = PgClientInternal::find_by_id(client.id(), &mut transaction).await?;

        println!("fetched time: {}ms", timer.elapsed().as_millis());
        println!("{:#?}", fetched);

        transaction.rollback().await?;

        Ok(())
    }
}