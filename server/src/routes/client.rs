use axum::{response::IntoResponse, extract::State, Json};

use crate::InteractionHandler;

use self::{
    form::RegistrationForm, 
    response::ErrorResponse
};

mod form;
mod response;

pub struct ClientRegistration;

impl ClientRegistration {
    pub async fn register(
        State(_handler): State<InteractionHandler>,
        Json(_form): Json<RegistrationForm>
    ) -> Result<impl IntoResponse, ErrorResponse> {
        Ok(())
    }
}