use axum::{
    headers::{HeaderMap, HeaderValue},
    Form,
    extract::State,
    http::{header::SET_COOKIE, StatusCode},
    response::IntoResponse
};
use application::{
    services::VerifyAccountService,
    transfer::account::VerifyAccountDto
};
use crate::{Handler, ServerError};
use crate::extract::session::{Session, SESSION_TAG};
use self::form::UserInput;

pub async fn login(
    State(handler): State<Handler>,
    session: Session,
    Form(input): Form<UserInput>
) -> Result<impl IntoResponse, ServerError> {
    let dto = VerifyAccountDto {
        address: input.address,
        pass: input.pass,
        ticket: input.code,
        session: session.into(),
    };

    let session = handler.verify(dto).await?;

    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, HeaderValue::from_str(format!("{}={}", SESSION_TAG, session.id).as_str())
        .map_err(|e| ServerError::Axum(anyhow::Error::new(e)))?);

    Ok((headers, StatusCode::OK))
}

mod form {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct UserInput {
        pub address: Option<String>,
        pub pass: Option<String>,
        pub code: Option<String>,
    }
}