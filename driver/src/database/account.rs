use kernel::{
    interfaces::repository::{AccountRepository, TemporaryAccountRepository},
    prelude::entities::{Account, UserId, NonVerifiedAccount, TicketId, Address},
    KernelError
};
use deadpool_redis::Connection as RedisConnection;
use deadpool_redis::redis;
use sqlx::{Pool, Postgres, PgConnection};
use kernel::external::{OffsetDateTime, Uuid};

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

    async fn find_by_address(&self, address: &Address) -> Result<Option<Account>, KernelError> {
        let mut con = self.pool.acquire().await
            .map_err(DriverError::SqlX)?;
        let found = PgAccountInternal::find_by_address(address, &mut con).await?;
        Ok(found)
    }
}

pub(in crate::database) struct PgAccountInternal;

#[allow(dead_code)]
#[derive(sqlx::FromRow)]
pub(in crate::database) struct AccountRow {
    user_id: Uuid,
    address: String,
    name: String,
    pass: String,
    verified_at: OffsetDateTime,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime
}

impl PgAccountInternal {
    pub async fn create(create: &Account, con: &mut PgConnection) -> Result<(), DriverError> {
        // language=SQL
        sqlx::query(r#"
            INSERT INTO users (
                user_id,
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
        .bind(AsRef::<Uuid>::as_ref(create.id()))
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
        // language=SQL
        sqlx::query(r#"
            UPDATE users
            SET
                address = $1,
                name = $2,
                pass = $3,
                updated_at = $4
            WHERE
                user_id = $5
        "#)
        .bind(update.address().as_ref())
        .bind(update.name().as_ref())
        .bind(update.pass().as_ref())
        .bind(update.date().updated_at().as_ref())
        .bind(AsRef::<Uuid>::as_ref(update.id()))
        .execute(&mut *con)
        .await?;

        Ok(())
    }

    pub async fn delete(delete: &UserId, con: &mut PgConnection) -> Result<(), DriverError> {
        sqlx::query(r#"
            DELETE FROM users WHERE id = $1
        "#)
        .bind(AsRef::<Uuid>::as_ref(delete))
        .execute(&mut *con)
        .await?;

        Ok(())
    }

    pub async fn find_by_id(id: &UserId, con: &mut PgConnection) -> Result<Option<Account>, DriverError> {
        // language=SQL
        sqlx::query_as::<_, AccountRow>(r#"
            SELECT * from users WHERE user_id = $1
        "#)
        .bind(AsRef::<Uuid>::as_ref(id))
        .fetch_optional(&mut *con)
        .await?
        .map(|fetched| -> Result<Account, DriverError> {
            Ok(Account::new_with_unchecked(
                fetched.user_id,
                fetched.address, 
                fetched.name, 
                fetched.pass, 
                fetched.created_at, 
                fetched.updated_at, 
                fetched.verified_at
            ))
        })
        .transpose()
    }

    pub async fn find_by_address(address: &Address, con: &mut PgConnection) -> Result<Option<Account>, DriverError> {
        // language=SQL
        sqlx::query_as::<_, AccountRow>(r#"
            SELECT * from users WHERE address LIKE $1
        "#)
        .bind(address.as_ref())
        .fetch_optional(&mut *con)
        .await?
        .map(|fetched| -> Result<Account, DriverError> {
            Ok(Account::new_with_unchecked(
                fetched.user_id,
                fetched.address,
                fetched.name,
                fetched.pass,
                fetched.created_at,
                fetched.updated_at,
                fetched.verified_at
            ))
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
impl TemporaryAccountRepository for NonVerifiedAccountDataBase {
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

    //noinspection DuplicatedCode
    pub async fn find_by_id(id: &TicketId, con: &mut RedisConnection) -> Result<Option<NonVerifiedAccount>, DriverError> {
        let Some(address) = redis::cmd("GET").arg(id.as_ref()).query_async::<_, Option<String>>(&mut *con).await? else {
            return Ok(None);
        };

        let Some(code) = redis::cmd("GET").arg(&address).query_async::<_, Option<String>>(&mut *con).await? else {
            return Ok(None);
        };

        Ok(Some(NonVerifiedAccount::new(id.clone(), address, code)))
    }

    //noinspection DuplicatedCode
    pub async fn find_by_valid_id(id: &TicketId, con: &mut RedisConnection) -> Result<Option<Address>, DriverError> {
        let Some(address) = redis::cmd("GET").arg(id.as_ref()).query_async::<_, Option<String>>(&mut *con).await? else {
            return Ok(None);
        };
        Ok(Some(Address::new(address)))
    }
}