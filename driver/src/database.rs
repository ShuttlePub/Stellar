mod account;
mod client;
mod tokens;
mod pkce;
mod state;
mod mfa_code;
mod session;
mod ticket;

pub use self::{
    account::*,
    client::*,
    tokens::*,
    pkce::*,
    state::*,
    mfa_code::*,
    session::*,
    ticket::*,
    redis_pool::*,
};

pub(in crate::database) mod redis_pool {
    use deadpool_redis::{Pool, Connection as RedisConnection};
    use crate::error::DriverError;
    
    pub struct RedisPoolMng;

    impl RedisPoolMng {
        pub async fn acquire(pool: &Pool) -> Result<RedisConnection, DriverError> {
            Ok(pool.get().await?)
        }
    }
}