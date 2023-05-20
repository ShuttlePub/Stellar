use deadpool_redis::{Pool, redis};
use deadpool_redis::Connection as RedisConnection;
use kernel::entities::{AuthorizeTokenId, CodeChallenge};
use kernel::KernelError;
use kernel::repository::PKCEVolatileRepository;
use crate::DriverError;

pub struct PKCEVolatileDataBase {
    pool: Pool
}

impl PKCEVolatileDataBase {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl PKCEVolatileRepository for PKCEVolatileDataBase {
    async fn save(&self, token_id: &AuthorizeTokenId, code: &CodeChallenge) -> Result<(), KernelError> {
        let mut con = self.pool.get().await
            .map_err(DriverError::from)?;
        PKCERedisInternal::save(token_id, code, &mut con).await?;
        Ok(())
    }

    async fn dele(&self, token_id: &AuthorizeTokenId) -> Result<(), KernelError> {
        let mut con = self.pool.get().await
            .map_err(DriverError::from)?;
        PKCERedisInternal::dele(token_id, &mut con).await?;
        Ok(())
    }

    async fn find(&self, token_id: &AuthorizeTokenId) -> Result<Option<CodeChallenge>, KernelError> {
        let mut con = self.pool.get().await
            .map_err(DriverError::from)?;
        let found = PKCERedisInternal::find(token_id, &mut con).await?;
        Ok(found)
    }
}

pub(in crate::database) struct PKCERedisInternal;

impl PKCERedisInternal {
    async fn save(
        token_id: &AuthorizeTokenId,
        code: &CodeChallenge,
        con: &mut RedisConnection
    ) -> Result<(), DriverError> {
        redis::cmd("SET")
            .arg(token_id.as_ref())
            .arg(code.as_ref())
            .query_async(&mut *con)
            .await?;
        Ok(())
    }

    async fn dele(
        token_id: &AuthorizeTokenId,
        con: &mut RedisConnection
    ) -> Result<(), DriverError> {
        redis::cmd("DEL")
            .arg(token_id.as_ref())
            .query_async(&mut *con)
            .await?;
        Ok(())
    }

    async fn find(
        token_id: &AuthorizeTokenId,
        con: &mut RedisConnection
    ) -> Result<Option<CodeChallenge>, DriverError> {
        let raw: Option<Vec<u8>> = redis::cmd("GET")
            .arg(token_id.as_ref())
            .query_async(&mut *con)
            .await?;
        let pkce = raw.map(CodeChallenge::from);
        Ok(pkce)
    }
}

#[cfg(test)]
mod tests {
    use deadpool_redis::{Config, Runtime};
    use kernel::entities::{AuthorizeTokenId, CodeChallenge};
    use crate::database::pkce::PKCERedisInternal;

    //noinspection SpellCheckingInspection
    #[ignore = "It depends on Redis and does not work as is."]
    #[tokio::test]
    async fn all() -> anyhow::Result<()> {
        let cfg = Config::from_url("redis://localhost:6379/");
        let pool = cfg.create_pool(Some(Runtime::Tokio1))?;

        const ORIGIN: &str = "3hZ3yzGLxZXUG2lPcUoJCtL14IuuE8y9YaH6idyuDMFVNzcg6jlI6IEzv7HrdUPIft0vlIcR4Us2AoP6GChEORHoBdMpcoLusoE79QlHwsqXRsoa6lVVTHVZSSUK1MeE";
        const ENCODE: &str = "R4SUkGMHJj_GM8aS5NUGrXbtF5_npYMbiJhPZAqgk9o=";

        let code = CodeChallenge::new(ENCODE)?;
        println!("{:?}", code);

        let token_id = AuthorizeTokenId::default();

        let mut con = pool.get().await?;

        PKCERedisInternal::save(&token_id, &code, &mut con).await?;
        let value = PKCERedisInternal::find(&token_id, &mut con).await?;
        assert!(value.is_some());
        println!("{:?}", value);

        let re = value.unwrap();
        assert_eq!(re, code);

        let re = re.verify(ORIGIN);
        assert!(re.is_ok());

        PKCERedisInternal::dele(&token_id, &mut con).await?;
        let value = PKCERedisInternal::find(&token_id, &mut con).await?;
        assert!(value.is_none());
        println!("{:?}", value);

        Ok(())
    }
}