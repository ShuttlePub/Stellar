use serde::{Deserialize, Serialize};
use kernel::entities::{Account, Address, CreatedAt, Password, UserId, UserName};
use crate::config::model::Admin;
use crate::DriverError;

#[derive(Debug, Clone, Hash, Deserialize, Serialize)]
pub struct AdminUser {
    user_id: UserId,
    name: UserName,
    address: Address,
    pass: Password,
}

impl AdminUser {
    pub fn user_id(&mut self, user_id: UserId) {
        self.user_id = user_id
    }
}

impl TryFrom<Admin> for AdminUser {
    type Error = DriverError;
    fn try_from(value: Admin) -> Result<Self, Self::Error> {
        Ok(Self {
            user_id: UserId::default(),
            name: UserName::new(value.name),
            address: Address::new(value.address),
            pass: value.pass_hashed
                .map(Password::unchecked_new)
                .unwrap_or(Password::new(value.pass)?),
        })
    }
}

impl TryFrom<AdminUser> for Account {
    type Error = DriverError;
    fn try_from(value: AdminUser) -> Result<Self, Self::Error> {
        let created_at = CreatedAt::default();
        Ok(Self::new(
            value.user_id, 
            value.address, 
            value.name, 
            value.pass, 
            created_at, 
            created_at, 
            created_at)?)
    }
}