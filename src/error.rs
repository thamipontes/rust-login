use axum::{response::{IntoResponse, Response}, http::StatusCode};
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    LoginFail,
    TicketDeleteFailNotFound {id : u64},
    AuthFailNoAuthTokenCookie,
    AuthFAilTokenWrongFormat,
    AuthFailsCtxNotInRequestExt,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        //INsert the Error into the response
        response.extensions_mut().insert(self);

        response
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {

        #[allow(unreachable_patterns)]
        match self {

            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

            //Auth
            Self::AuthFailNoAuthTokenCookie | Self::AuthFAilTokenWrongFormat
            | Self::AuthFailsCtxNotInRequestExt => {
                (StatusCode::FORBIDDEN, ClientError::NO_AUTH)
            }

            // Model
            Self::TicketDeleteFailNotFound {..} => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            }

            // Fallback
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            )
        }
    }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}