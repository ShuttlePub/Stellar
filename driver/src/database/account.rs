use kernel::{
    repository::{AccountRepository, NonVerifiedAccountRepository},
    entities::{Account, UserId, NonVerifiedAccount, TicketId, Address},
    KernelError
};
use deadpool_redis::Connection as RedisConnection;
use deadpool_redis::redis;
use sqlx::{Pool, Postgres, PgConnection};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::DriverError;

#[derive(Debug, Clone)]
pub struct AccountDataBase {
    pool: Pool<Postgres>
}

impl AccountDataBase {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl AccountRepository for AccountDataBase {
    async fn create(&self, create: &Account) -> Result<(), KernelError> {
        let mut con = self.pool.acquire().await
            .map_err(DriverError::SqlX)?;
        PgAccountInternal::create(create, &mut con).await?;

        Ok(())
    }

    async fn update(&self, update: &Account) -> Result<(), KernelError> {
        let mut con = self.pool.acquire().await
            .map_err(DriverError::SqlX)?;
        PgAccountInternal::update(update, &mut con).await?;

        Ok(())
    }

    async fn delete(&self, delete: &UserId) -> Result<(), KernelError> {
        let mut con = self.pool.acquire().await
            .map_err(DriverError::SqlX)?;
        PgAccountInternal::delete(delete, &mut con).await?;

        Ok(())
    }

    async fn find_by_id(&self, id: &UserId) -> Result<Option<Account>, KernelError> {
        let mut con = self.pool.acquire().await
            .map_err(DriverError::SqlX)?;
        let found = PgAccountInternal::find_by_id(id, &mut con).await?;
        Ok(found)
    }
}

pub(in crate::database) struct PgAccountInternal;

#[allow(dead_code)]
#[derive(sqlx::FromRow)]
pub(in crate::database) struct AccountRow {
    id: Uuid,
    address: String,
    name: String,
    pass: String,
    verified_at: OffsetDateTime,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime
}

impl PgAccountInternal {
    pub async fn create(create: &Account, con: &mut PgConnection) -> Result<(), DriverError> {
        sqlx::query(r#"
            INSERT INTO users (
                id,
                address,
                name,
                pass,
                verified_at,
                created_at,
                updated_at
            )
            VALUES (
                $1,
                $2,
                $3,
                $4,
                $5,
                $6,
                $7
            );
        "#)
        .bind(create.id().as_ref())
        .bind(create.address().as_ref())
        .bind(create.name().as_ref())
        .bind(create.pass().as_ref())
        .bind(create.verified_at().as_ref())
        .bind(create.date().created_at().as_ref())
        .bind(create.date().updated_at().as_ref())
        .execute(&mut *con)
        .await?;

        Ok(())
    }

    pub async fn update(update: &Account, con: &mut PgConnection) -> Result<(), DriverError> {
        sqlx::query(r#"
            UPDATE users
            SET
                address = $1,
                name = $2,
                pass = $3,
                updated_at = $4
            WHERE
                id = $5
        "#)
        .bind(update.address().as_ref())
        .bind(update.name().as_ref())
        .bind(update.pass().as_ref())
        .bind(update.date().updated_at().as_ref())
        .bind(update.id().as_ref())
        .execute(&mut *con)
        .await?;

        Ok(())
    }

    pub async fn delete(delete: &UserId, con: &mut PgConnection) -> Result<(), DriverError> {
        sqlx::query(r#"
            DELETE FROM users WHERE id = $1
        "#)
        .bind(delete.as_ref())
        .execute(&mut *con)
        .await?;

        Ok(())
    }

    pub async fn find_by_id(id: &UserId, con: &mut PgConnection) -> Result<Option<Account>, DriverError> {
        sqlx::query_as::<_, AccountRow>(r#"
            SELECT * from users WHERE id = $1
        "#)
        .bind(id.as_ref())
        .fetch_optional(&mut *con)
        .await?
        .map(|fetched| -> Result<Account, DriverError> {
            Ok(Account::new(
                fetched.id, 
                fetched.address, 
                fetched.name, 
                fetched.pass, 
                fetched.created_at, 
                fetched.updated_at, 
                fetched.verified_at
            )?)
        })
        .transpose()
    }
}

#[derive(Clone)]
pub struct NonVerifiedAccountDataBase {
    pool: deadpool_redis::Pool
}

impl NonVerifiedAccountDataBase {
    pub fn new(pool: deadpool_redis::Pool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl NonVerifiedAccountRepository for NonVerifiedAccountDataBase {
    async fn create(&self, create: &NonVerifiedAccount) -> Result<(), KernelError> {
        let mut con = self.pool.get().await
            .map_err(DriverError::from)?;
        RedisInternal::create(create, &mut con).await?;
        
        Ok(())
    }

    async fn validation(&self, coupon: &TicketId, valid: &TicketId, address: &Address) -> Result<(), KernelError> {
        let mut con = self.pool.get().await
            .map_err(DriverError::from)?;
        RedisInternal::validation(coupon, valid, address, &mut con).await?;
        Ok(())
    }

    async fn find_by_id(&self, id: &TicketId) -> Result<Option<NonVerifiedAccount>, KernelError> {
        let mut con = self.pool.get().await
            .map_err(DriverError::from)?;
        let fetch = RedisInternal::find_by_id(id, &mut con).await?;
        Ok(fetch)
    }

    async fn find_by_valid_id(&self, id: &TicketId) -> Result<Option<Address>, KernelError> {
        let mut con = self.pool.get().await
            .map_err(DriverError::from)?;
        let valid = RedisInternal::find_by_valid_id(id, &mut con).await?;
        Ok(valid)
    }
}

pub(in crate::database) struct RedisInternal;

impl RedisInternal {
    pub async fn create(create: &NonVerifiedAccount, con: &mut RedisConnection) -> Result<(), DriverError> {
        redis::pipe()
            .cmd("SET")
                .arg(create.id().as_ref())
                .arg(create.address().as_ref())
            .arg("EX")
                .arg(6000)
            .ignore()
            .cmd("SET")
                .arg(create.address().as_ref())
                .arg(create.code().as_ref())
            .arg("EX")
                .arg(6010)
            .query_async(&mut *con)
            .await?;

        Ok(())
    }

    pub async fn validation(coupon: &TicketId, valid: &TicketId, address: &Address, con: &mut RedisConnection) -> Result<(), DriverError> {
        redis::cmd("DEL").arg(coupon.as_ref()).query_async(&mut *con).await?;
        redis::cmd("SET").arg(valid.as_ref()).arg(address.as_ref()).arg("EX").arg("6000").query_async(&mut *con).await?;
        Ok(())
    }

    pub async fn find_by_id(id: &TicketId, con: &mut RedisConnection) -> Result<Option<NonVerifiedAccount>, DriverError> {
        let Some(address) = redis::cmd("GET").arg(id.as_ref()).query_async::<_, Option<String>>(&mut *con).await? else {
            return Ok(None);
        };

        let Some(code) = redis::cmd("GET").arg(&address).query_async::<_, Option<String>>(&mut *con).await? else {
            return Ok(None);
        };

        Ok(Some(NonVerifiedAccount::new(id.clone(), address, code)))
    }

    pub async fn find_by_valid_id(id: &TicketId, con: &mut RedisConnection) -> Result<Option<Address>, DriverError> {
        let Some(address) = redis::cmd("GET").arg(id.as_ref()).query_async::<_, Option<String>>(&mut *con).await? else {
            return Ok(None);
        };
        Ok(Some(Address::new(address)))
    }
}