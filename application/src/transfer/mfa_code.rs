use kernel::prelude::entities::TicketId;

#[derive(Debug)]
pub struct MFAActionDto {
    pub pending: String,
    pub code: String
}

#[derive(Debug)]
pub struct TicketIdDto(pub String);

impl From<TicketId> for TicketIdDto {
    fn from(value: TicketId) -> Self {
        Self(value.into())
    }
}
