use deadpool_redis::{Pool, redis, Connection as RedisConnection};
use kernel::prelude::entities::{Session, SessionId};
use kernel::KernelError;
use kernel::interfaces::repository::SessionVolatileRepository;
use crate::DriverError;

#[derive(Clone)]
pub struct SessionVolatileDataBase {
    pool: Pool
}

impl SessionVolatileDataBase {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl SessionVolatileRepository for SessionVolatileDataBase {
    async fn establish(&self, session: &Session) -> Result<(), KernelError> {
        let mut con = self.pool.get().await
            .map_err(DriverError::from)?;
        SessionRedisInternal::create(session, &mut con).await?;
        Ok(())
    }

    async fn revoke(&self, id: &SessionId) -> Result<(), KernelError> {
        let mut con = self.pool.get().await
            .map_err(DriverError::from)?;
        SessionRedisInternal::delete(id, &mut con).await?;
        Ok(())
    }

    async fn find(&self, id: &SessionId) -> Result<Option<Session>, KernelError> {
        let mut con = self.pool.get().await
            .map_err(DriverError::from)?;
        let found = SessionRedisInternal::find(id, &mut con).await?;
        Ok(found)
    }
}

pub(in crate::database) struct SessionRedisInternal;

impl SessionRedisInternal {
    async fn create(
        session: &Session,
        con: &mut RedisConnection
    ) -> Result<(), DriverError> {
        redis::cmd("SET")
            .arg(session.id().as_ref())
            .arg(serde_json::to_string(session)?)
            .query_async(&mut *con)
            .await?;
        Ok(())
    }

    async fn delete(
        id: &SessionId,
        con: &mut RedisConnection
    ) -> Result<(), DriverError> {
        redis::cmd("DEL")
            .arg(id.as_ref())
            .query_async(&mut *con)
            .await?;
        Ok(())
    }

    async fn find(
        id: &SessionId,
        con: &mut RedisConnection
    ) -> Result<Option<Session>, DriverError> {
        let raw: Option<String> = redis::cmd("GET")
            .arg(id.as_ref())
            .query_async(&mut *con)
            .await?;
        let session = raw.map(|s| serde_json::from_str::<Session>(&s))
            .transpose()?;
        Ok(session)
    }
}