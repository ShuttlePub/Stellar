use application::services::{DependOnVerifyMFACodeService, VerifyMFACodeService};
use application::transfer::mfa_code::MFAActionDto;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use self::forms::*;
use crate::{Handler, ServerError};

pub async fn verify(
    State(handler): State<Handler>,
    Query(query): Query<UserQuery>,
    Json(form): Json<UserInput>,
) -> Result<impl IntoResponse, ServerError> {
    let dto = MFAActionDto {
        pending: query.ticket,
        code: form.code,
    };

    let accepted = handler.verify_mfa_code_service().verify(dto).await?;

    let res = Response { ticket: accepted.0 };

    Ok((StatusCode::TEMPORARY_REDIRECT, Json(res)))
}

mod forms {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Debug)]
    pub struct UserInput {
        pub code: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct UserQuery {
        pub ticket: String,
    }

    #[derive(Serialize, Debug)]
    pub struct Response {
        pub ticket: String,
    }
}
