use deadpool_redis::{Pool, redis, Connection as RedisConnection};
use kernel::prelude::entities::{AuthorizeToken, AuthorizeTokenId};
use kernel::interfaces::repository::AuthorizeTokenRepository;
use kernel::KernelError;
use crate::DriverError;

#[derive(Clone)]
pub struct AuthorizeTokenVolatileDataBase {
    pool: Pool
}

impl AuthorizeTokenVolatileDataBase {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl AuthorizeTokenRepository for AuthorizeTokenVolatileDataBase {
    async fn save(&self, id: &AuthorizeTokenId, token: &AuthorizeToken) -> Result<(), KernelError> {
        let mut con = self.pool.get().await
            .map_err(DriverError::from)?;
        AuthorizeTokenRedisInternal::save(id, token, &mut con).await?;
        Ok(())
    }

    async fn dele(&self, id: &AuthorizeTokenId) -> Result<(), KernelError> {
        let mut con = self.pool.get().await
            .map_err(DriverError::from)?;
        AuthorizeTokenRedisInternal::dele(id, &mut con).await?;
        Ok(())
    }

    async fn find(&self, id: &AuthorizeTokenId) -> Result<Option<AuthorizeToken>, KernelError> {
        let mut con = self.pool.get().await
            .map_err(DriverError::from)?;
        let found = AuthorizeTokenRedisInternal::find(id, &mut con).await?;
        Ok(found)
    }
}

pub(in crate::database) struct AuthorizeTokenRedisInternal;

//noinspection DuplicatedCode
impl AuthorizeTokenRedisInternal {
    pub async fn save(
        id: &AuthorizeTokenId,
        token: &AuthorizeToken,
        con: &mut RedisConnection
    ) -> Result<(), DriverError> {
        redis::cmd("SET")
            .arg(namespace(id))
            .arg(serde_json::to_string(token)?)
            .arg("EX")
            .arg(token.context().expired_in().as_ref_i64())
            .query_async(&mut *con)
            .await?;
        Ok(())
    }

    pub async fn dele(
        id: &AuthorizeTokenId,
        con: &mut RedisConnection
    ) -> Result<(), DriverError> {
        redis::cmd("DEL")
            .arg(namespace(id))
            .query_async(&mut *con)
            .await?;
        Ok(())
    }

    pub async fn find(
        id: &AuthorizeTokenId,
        con: &mut RedisConnection
    ) -> Result<Option<AuthorizeToken>, DriverError> {
        let raw: Option<String> = redis::cmd("GET")
            .arg(namespace(id))
            .query_async(&mut *con)
            .await?;
        let token = raw
            .map(|raw| serde_json::from_str::<AuthorizeToken>(&raw))
            .transpose()?;
        Ok(token)
    }
}

fn namespace(key: impl AsRef<str>) -> String {
    format!("{}-accepted-authz-token", key.as_ref())
}


#[cfg(test)]
mod tests {
    use deadpool_redis::{Config, Runtime};
    use kernel::prelude::entities::{AuthorizeToken, AuthorizeTokenId, ClientId, ResponseType, ScopeMethod};
    use kernel::external::{Duration, OffsetDateTime, Uuid};
    use super::AuthorizeTokenRedisInternal;

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

        AuthorizeTokenRedisInternal::save(token.id(), &token, &mut con).await?;

        let found = AuthorizeTokenRedisInternal::find(token.id(), &mut con).await?;

        println!("{:?}", found);
        assert_eq!(Some(token.clone()), found);

        AuthorizeTokenRedisInternal::dele(token.id(), &mut con).await?;

        let found = AuthorizeTokenRedisInternal::find(token.id(), &mut con).await?;
        println!("{:?}", found);
        assert_ne!(Some(token), found);

        Ok(())
    }
}