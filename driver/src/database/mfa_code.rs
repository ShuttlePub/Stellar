use deadpool_redis::{Pool, Connection as RedisConnection, redis};
use kernel::entities::{MFACode, UserId};
use kernel::KernelError;
use kernel::repository::MFACodeVolatileRepository;
use crate::DriverError;

#[derive(Clone)]
pub struct MFACodeVolatileDataBase {
    pool: Pool
}

impl MFACodeVolatileDataBase {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl MFACodeVolatileRepository for MFACodeVolatileDataBase {
    async fn create(&self, user_id: &UserId, code: &MFACode) -> Result<(), KernelError> {
        let mut con = self.pool.get().await
            .map_err(DriverError::from)?;
        MFACodeRedisInternal::create(user_id, code, &mut con).await?;
        Ok(())
    }

    async fn delete(&self, user_id: &UserId) -> Result<(), KernelError> {
        let mut con = self.pool.get().await
            .map_err(DriverError::from)?;
        MFACodeRedisInternal::delete(user_id, &mut con).await?;
        Ok(())
    }

    async fn find_by_id(&self, user_id: &UserId) -> Result<Option<MFACode>, KernelError> {
        let mut con = self.pool.get().await
            .map_err(DriverError::from)?;
        let found = MFACodeRedisInternal::find_by_id(user_id, &mut con).await?;
        Ok(found)
    }
}

pub(in crate::database) struct MFACodeRedisInternal;

impl MFACodeRedisInternal {
    async fn create(
        user_id: &UserId,
        code: &MFACode,
        con: &mut RedisConnection
    ) -> Result<(), DriverError> {
        redis::cmd("SET")
            .arg(AsRef::<[u8]>::as_ref(user_id))
            .arg(code.as_ref())
        .arg("EX")
            .arg(60 * 60)
            .query_async(&mut *con)
            .await?;
        Ok(())
    }

    async fn delete(
        user_id: &UserId,
        con: &mut RedisConnection
    ) -> Result<(), DriverError> {
        redis::cmd("DEL")
            .arg(AsRef::<[u8]>::as_ref(user_id))
            .query_async(&mut *con)
            .await?;
        Ok(())
    }

    async fn find_by_id(
        user_id: &UserId,
        con: &mut RedisConnection
    ) -> Result<Option<MFACode>, DriverError> {
        let code: Option<String> = redis::cmd("GET")
            .arg(AsRef::<[u8]>::as_ref(user_id))
            .query_async(&mut *con)
            .await?;
        let code = code.map(MFACode::new);
        Ok(code)
    }
}