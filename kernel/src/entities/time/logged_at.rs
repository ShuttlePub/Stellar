use destructure::Destructure;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use super::{CreatedAt, UpdatedAt};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize, Destructure)]
pub struct LoggedAt {
    created_at: CreatedAt,
    updated_at: UpdatedAt
}

impl LoggedAt {
    pub fn new(
        created_at: impl Into<OffsetDateTime>,
        updated_at: impl Into<OffsetDateTime>
    ) -> Self {
        Self {
            created_at: CreatedAt::new(created_at.into()),
            updated_at: UpdatedAt::new(updated_at.into())
        }
    }

    pub fn created_at(&self) -> &CreatedAt {
        &self.created_at
    }

    pub fn updated_at(&self) -> &UpdatedAt {
        &self.updated_at
    }
}
