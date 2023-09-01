mod account;
mod client;
mod mfa_code;
mod pkce;
mod session;
mod state;
mod ticket;
mod tokens;

pub use self::{
    account::*, client::*, mfa_code::*, pkce::*, redis_pool::*, session::*, state::*, ticket::*,
    tokens::*,
};

pub(in crate::database) mod redis_pool {
    use crate::error::DriverError;
    use deadpool_redis::{Connection as RedisConnection, Pool};

    pub struct RedisPoolMng;

    impl RedisPoolMng {
        pub async fn acquire(pool: &Pool) -> Result<RedisConnection, DriverError> {
            Ok(pool.get().await?)
        }
    }
}
