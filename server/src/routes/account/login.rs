use self::form::*;
use crate::controller::Controller;
use crate::extract::session::Session;
use crate::{Handler, ServerError};
use application::services::{DependOnVerifyAccountService, VerifyAccountService};
use axum::{extract::State, response::IntoResponse, Form};

pub async fn login(
    State(handler): State<Handler>,
    session: Session,
    Form(input): Form<UserInput>,
) -> Result<impl IntoResponse, ServerError> {
    let req = Request {
        address: input.address,
        pass: input.pass,
        code: input.code,
        session: session.into(),
    };

    let res = Controller::new(Transformer, Presenter)
        .transform(req)
        .handle(|req| async { handler.verify_account_service().verify(req).await })
        .await?;

    Ok(res)
}

mod form {
    use crate::{
        controller::{InputPort, OutputPort},
        extract::session::SESSION_TAG,
        ServerError,
    };
    use application::{
        transfer::account::VerifyAccountDto, transfer::session::SessionDto, ApplicationError,
    };
    use axum::{
        http::header::SET_COOKIE,
        http::{HeaderMap, StatusCode},
    };
    use axum_extra::headers::HeaderValue;
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct UserInput {
        pub address: Option<String>,
        pub pass: Option<String>,
        pub code: Option<String>,
    }

    pub struct Request {
        pub address: Option<String>,
        pub pass: Option<String>,
        pub code: Option<String>,
        pub session: Option<String>,
    }

    pub struct Transformer;

    impl InputPort<Request> for Transformer {
        type Dto = VerifyAccountDto;
        fn emit(&self, input: Request) -> Self::Dto {
            VerifyAccountDto {
                address: input.address,
                pass: input.pass,
                ticket: input.code,
                session: input.session,
            }
        }
    }

    pub struct Presenter;

    impl OutputPort<Result<SessionDto, ApplicationError>> for Presenter {
        type ViewModel = Result<(HeaderMap, StatusCode), ServerError>;
        fn emit(&self, input: Result<SessionDto, ApplicationError>) -> Self::ViewModel {
            match input {
                Ok(session) => {
                    let session =
                        HeaderValue::from_str(format!("{}={}", SESSION_TAG, session.id).as_str())
                            .map_err(|e| ServerError::Axum(anyhow::Error::new(e)))?;

                    let mut headers = HeaderMap::new();
                    headers.insert(SET_COOKIE, session);

                    Ok((headers, StatusCode::OK))
                }
                Err(e) => Err(e.into()),
            }
        }
    }
}
