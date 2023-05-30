use axum::{response::IntoResponse, extract::State, Json, TypedHeader};
use axum::headers::Authorization;
use axum::headers::authorization::Bearer;
use application::services::{
    DependOnRegisterClientService,
    RegisterClientService
};

use crate::Handler;

use self::{
    form::RegistrationForm, 
    response::ErrorResponse
};

mod form;
mod response;

pub struct ClientRegistration;

impl ClientRegistration {
    pub async fn register(
        TypedHeader(_header): TypedHeader<Authorization<Bearer>>,
        State(_handler): State<Handler>,
        Json(_form): Json<RegistrationForm>
    ) -> Result<impl IntoResponse, ErrorResponse> {
        //_handler.register_client_service().register(_form.convert_dto())
        Ok(())
    }
}