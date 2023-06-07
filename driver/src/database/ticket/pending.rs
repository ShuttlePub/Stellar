use deadpool_redis::{Pool, Connection as RedisConnection, redis};
use kernel::entities::{TicketId, UserId};
use kernel::KernelError;
use kernel::repository::PendingActionVolatileRepository;
use crate::DriverError;

#[derive(Clone)]
pub struct PendingActionVolatileDataBase {
    pool: Pool
}

impl PendingActionVolatileDataBase {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl PendingActionVolatileRepository for PendingActionVolatileDataBase {
    async fn create(&self, ticket: &TicketId, user_id: &UserId) -> Result<(), KernelError> {
        let mut con = self.pool.get().await
            .map_err(DriverError::from)?;
        PendingActionRedisInternal::create(ticket, user_id, &mut con).await?;
        Ok(())
    }

    async fn revoke(&self, ticket: &TicketId) -> Result<(), KernelError> {
        let mut con = self.pool.get().await
            .map_err(DriverError::from)?;
        PendingActionRedisInternal::revoke(ticket, &mut con).await?;
        Ok(())
    }

    async fn find(&self, ticket: &TicketId) -> Result<Option<UserId>, KernelError> {
        let mut con = self.pool.get().await
            .map_err(DriverError::from)?;
        let found = PendingActionRedisInternal::find(ticket, &mut con).await?;
        Ok(found)
    }
}


pub(in crate::database) struct PendingActionRedisInternal;

// noinspection DuplicatedCode
impl PendingActionRedisInternal {
    pub async fn create(
        ticket: &TicketId,
        user_id: &UserId,
        con: &mut RedisConnection
    ) -> Result<(), DriverError> {
        redis::cmd("SET")
            .arg(ticket.as_ref())
            .arg(AsRef::<[u8]>::as_ref(user_id))
            .arg("EX")
            .arg(60 * 10)
            .query_async(&mut *con)
            .await?;

        Ok(())
    }

    pub async fn revoke(
        ticket: &TicketId,
        con: &mut RedisConnection
    ) -> Result<(), DriverError> {
        redis::cmd("DEL")
            .arg(ticket.as_ref())
            .query_async(&mut *con)
            .await?;
        Ok(())
    }

    pub async fn find(
        ticket: &TicketId,
        con: &mut RedisConnection
    ) -> Result<Option<UserId>, DriverError> {
        let raw: Option<String> = redis::cmd("GET")
            .arg(ticket.as_ref())
            .query_async(&mut *con)
            .await?;
        let user_id = raw
            .map(UserId::try_from)
            .transpose()?;
        Ok(user_id)
    }
}