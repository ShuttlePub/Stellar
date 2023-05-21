use deadpool_redis::{Pool, Connection as RedisConnection, redis};
use kernel::entities::{AuthorizeToken, TicketId};
use kernel::repository::PendingAuthorizeTokenRepository;
use kernel::KernelError;
use crate::DriverError;

#[derive(Clone)]
pub struct PendingAuthorizeTokenVolatileDataBase {
    pool: Pool
}

impl PendingAuthorizeTokenVolatileDataBase {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl PendingAuthorizeTokenRepository for PendingAuthorizeTokenVolatileDataBase {
    async fn save(&self, ticket: &TicketId, create: &AuthorizeToken) -> Result<(), KernelError> {
        let mut con = self.pool.get().await
            .map_err(DriverError::from)?;
        PendingAuthorizeTokenRedisInternal::save(ticket, create, &mut con).await?;
        Ok(())
    }

    async fn dele(&self, ticket: &TicketId) -> Result<(), KernelError> {
        let mut con = self.pool.get().await
            .map_err(DriverError::from)?;
        PendingAuthorizeTokenRedisInternal::dele(ticket, &mut con).await?;
        Ok(())
    }

    async fn find(&self, ticket: &TicketId) -> Result<Option<AuthorizeToken>, KernelError> {
        let mut con = self.pool.get().await
            .map_err(DriverError::from)?;
        let found = PendingAuthorizeTokenRedisInternal::find(ticket, &mut con).await?;
        Ok(found)
    }
}

pub(in crate::database) struct PendingAuthorizeTokenRedisInternal;

//noinspection DuplicatedCode
impl PendingAuthorizeTokenRedisInternal {
    pub async fn save(
        ticket: &TicketId,
        create: &AuthorizeToken,
        con: &mut RedisConnection
    ) -> Result<(), DriverError> {
        redis::cmd("SET")
            .arg(namespace(ticket))
            .arg(serde_json::to_string(create)?)
            .arg("EX")
            .arg(create.context().expired_in().as_ref_i64())
            .query_async(&mut *con)
            .await?;
        Ok(())
    }

    //noinspection DuplicatedCode
    pub async fn dele(
        ticket: &TicketId,
        con: &mut RedisConnection
    ) -> Result<(), DriverError> {
        redis::cmd("DEL")
            .arg(namespace(ticket))
            .query_async(&mut *con)
            .await?;
        Ok(())
    }

    pub async fn find(
        ticket: &TicketId,
        con: &mut RedisConnection
    ) -> Result<Option<AuthorizeToken>, DriverError> {
        let raw: Option<String> = redis::cmd("GET")
            .arg(namespace(ticket))
            .query_async(&mut *con)
            .await?;
        let token: Option<AuthorizeToken> = raw
            .map(|raw| serde_json::from_str::<AuthorizeToken>(&raw))
            .transpose()?;
        Ok(token)
    }
}

fn namespace(key: impl AsRef<str>) -> String {
    format!("{}-authz-token", key.as_ref())
}


#[cfg(test)]
mod tests {
    use deadpool_redis::{Config, Runtime};
    use kernel::entities::{AuthorizeToken, AuthorizeTokenId, ClientId, ResponseType, ScopeMethod, TicketId};
    use kernel::external::{Duration, OffsetDateTime, Uuid};
    use super::PendingAuthorizeTokenRedisInternal;

    #[ignore = "It depends on Redis and does not work as is."]
    #[tokio::test]
    async fn all() -> anyhow::Result<()> {
        let token_id = AuthorizeTokenId::default();
        let created_at = OffsetDateTime::now_utc();
        let updated_at = created_at;
        let expired_in = Duration::new(600, 0);
        let client_id = ClientId::new_at_now(Uuid::new_v4());
        let scopes = vec!["read", "write", "extras"]
            .into_iter()
            .map(ScopeMethod::new)
            .collect::<Vec<ScopeMethod>>();
        let response_type = ResponseType::Code;
        let redirect_uri = "https://client.example.com/callback";

        let token = AuthorizeToken::new(
            token_id,
            created_at,
            updated_at,
            None,
            client_id,
            scopes,
            response_type,
            redirect_uri,
            expired_in
        );

        let cfg = Config::from_url("redis://localhost:6379/");
        let pool = cfg.create_pool(Some(Runtime::Tokio1))?;
        let mut con = pool.get().await?;

        let ticket = TicketId::default();

        PendingAuthorizeTokenRedisInternal::save(&ticket, &token, &mut con).await?;

        let found = PendingAuthorizeTokenRedisInternal::find(&ticket, &mut con).await?;

        println!("{:?}", found);
        assert_eq!(Some(token.clone()), found);

        PendingAuthorizeTokenRedisInternal::dele(&ticket, &mut con).await?;

        let found = PendingAuthorizeTokenRedisInternal::find(&ticket, &mut con).await?;
        println!("{:?}", found);
        assert_ne!(Some(token), found);

        Ok(())
    }
}
