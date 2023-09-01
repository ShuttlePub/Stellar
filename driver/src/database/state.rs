use crate::DriverError;
use deadpool_redis::{redis, Connection as RedisConnection, Pool};
use kernel::interfaces::repository::StateVolatileRepository;
use kernel::prelude::entities::{State, TicketId};
use kernel::KernelError;

#[derive(Clone)]
pub struct StateVolatileDataBase {
    pool: Pool,
}

impl StateVolatileDataBase {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl StateVolatileRepository for StateVolatileDataBase {
    async fn save(&self, ticket: &TicketId, state: &State) -> Result<(), KernelError> {
        let mut con = self.pool.get().await.map_err(DriverError::from)?;
        StateRedisInternal::save(ticket, state, &mut con).await?;
        Ok(())
    }

    async fn dele(&self, ticket: &TicketId) -> Result<(), KernelError> {
        let mut con = self.pool.get().await.map_err(DriverError::from)?;
        StateRedisInternal::dele(ticket, &mut con).await?;
        Ok(())
    }

    async fn find(&self, ticket: &TicketId) -> Result<Option<State>, KernelError> {
        let mut con = self.pool.get().await.map_err(DriverError::from)?;
        let found = StateRedisInternal::find(ticket, &mut con).await?;
        Ok(found)
    }
}

pub(in crate::database) struct StateRedisInternal;

impl StateRedisInternal {
    pub async fn save(
        ticket: &TicketId,
        state: &State,
        con: &mut RedisConnection,
    ) -> Result<(), DriverError> {
        redis::cmd("SET")
            .arg(ticket.as_ref())
            .arg(state.as_ref())
            .query_async(&mut *con)
            .await?;
        Ok(())
    }

    //noinspection DuplicatedCode
    pub async fn dele(ticket: &TicketId, con: &mut RedisConnection) -> Result<(), DriverError> {
        redis::cmd("DEL")
            .arg(namespace(ticket))
            .query_async(&mut *con)
            .await?;
        Ok(())
    }

    pub async fn find(
        ticket: &TicketId,
        con: &mut RedisConnection,
    ) -> Result<Option<State>, DriverError> {
        let raw: Option<String> = redis::cmd("GET")
            .arg(ticket.as_ref())
            .query_async(&mut *con)
            .await?;
        let state = raw.map(State::new);
        Ok(state)
    }
}

fn namespace(key: impl AsRef<str>) -> String {
    format!("{}-state", key.as_ref())
}

#[cfg(test)]
mod tests {
    use crate::database::state::StateRedisInternal;
    use deadpool_redis::{Config, Runtime};
    use kernel::prelude::entities::{State, TicketId};

    #[ignore = "It depends on Redis and does not work as is."]
    #[tokio::test]
    async fn all() -> anyhow::Result<()> {
        let ticket = TicketId::default();
        let state = State::new("abc123");

        let cfg = Config::from_url("redis://localhost:6379/");
        let pool = cfg.create_pool(Some(Runtime::Tokio1))?;

        let mut con = pool.get().await?;

        StateRedisInternal::save(&ticket, &state, &mut con).await?;

        let found = StateRedisInternal::find(&ticket, &mut con).await?;

        println!("{:?}", found);
        assert_eq!(Some(state.clone()), found);

        StateRedisInternal::dele(&ticket, &mut con).await?;

        let found = StateRedisInternal::find(&ticket, &mut con).await?;

        println!("{:?}", found);
        assert_ne!(Some(state), found);
        Ok(())
    }
}
