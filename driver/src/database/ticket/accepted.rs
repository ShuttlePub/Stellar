use deadpool_redis::{Pool, Connection as RedisConnection, redis};
use kernel::prelude::entities::{TicketId, UserId};
use kernel::KernelError;
use kernel::interfaces::repository::AcceptedActionVolatileRepository;
use crate::database::RedisPoolMng;
use crate::DriverError;

#[derive(Clone)]
pub struct AcceptedActionVolatileDataBase {
    pool: Pool
}

impl AcceptedActionVolatileDataBase {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl AcceptedActionVolatileRepository for AcceptedActionVolatileDataBase {
    async fn create(&self, ticket: &TicketId, user_id: &UserId) -> Result<(), KernelError> {
        let mut con = RedisPoolMng::acquire(&self.pool).await?;
        AcceptedActionRedisInternal::create(ticket, user_id, &mut con).await?;
        Ok(())
    }

    async fn revoke(&self, ticket: &TicketId) -> Result<(), KernelError> {
        let mut con = RedisPoolMng::acquire(&self.pool).await?;
        AcceptedActionRedisInternal::revoke(ticket, &mut con).await?;
        Ok(())
    }

    async fn find(&self, ticket: &TicketId) -> Result<Option<UserId>, KernelError> {
        let mut con = RedisPoolMng::acquire(&self.pool).await?;
        let found = AcceptedActionRedisInternal::find(ticket, &mut con).await?;
        Ok(found)
    }
}


pub(in crate::database) struct AcceptedActionRedisInternal;

// noinspection DuplicatedCode
impl AcceptedActionRedisInternal {
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
        let found = raw
            .map(TryInto::try_into)
            .transpose()?;
        Ok(found)
    }
}

fn namespace(ticket: impl AsRef<str>) -> String {
    format!("{}-accepted", ticket.as_ref())
}