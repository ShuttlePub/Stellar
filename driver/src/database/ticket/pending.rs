use deadpool_redis::{Pool, Connection as RedisConnection, redis};
use kernel::prelude::entities::{TicketId, UserId};
use kernel::KernelError;
use kernel::interfaces::repository::PendingActionVolatileRepository;
use crate::database::RedisPoolMng;
use crate::DriverError;

#[derive(Clone)]
pub struct PendingActionVolatileDataBase {
    pool: Pool
}

impl PendingActionVolatileDataBase {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn acquire(&self) -> Result<RedisConnection, DriverError> {
        RedisPoolMng::acquire(&self.pool).await
    }
}

#[async_trait::async_trait]
impl PendingActionVolatileRepository for PendingActionVolatileDataBase {
    async fn create(&self, ticket: &TicketId, user_id: &UserId) -> Result<(), KernelError> {
        let mut con = self.acquire().await?;
        PendingActionRedisInternal::create(ticket, user_id, &mut con).await?;
        Ok(())
    }

    async fn revoke(&self, ticket: &TicketId) -> Result<(), KernelError> {
        let mut con = self.acquire().await?;
        PendingActionRedisInternal::revoke(ticket, &mut con).await?;
        Ok(())
    }

    async fn find(&self, ticket: &TicketId) -> Result<Option<UserId>, KernelError> {
        let mut con = self.acquire().await?;
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
            .arg(namespace(ticket))
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
            .arg(namespace(ticket))
            .query_async(&mut *con)
            .await?;
        Ok(())
    }

    pub async fn find(
        ticket: &TicketId,
        con: &mut RedisConnection
    ) -> Result<Option<UserId>, DriverError> {
        let raw: Option<Vec<u8>> = redis::cmd("GET")
            .arg(namespace(ticket))
            .query_async(&mut *con)
            .await?;
        let user_id = raw
            .map(TryInto::try_into)
            .transpose()?;
        Ok(user_id)
    }
}

fn namespace(ticket: impl AsRef<str>) -> String {
    format!("{}-pending", ticket.as_ref())
}
