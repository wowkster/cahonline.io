use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use thiserror::Error;

#[allow(unused)]
#[derive(Debug, Error)]
pub enum Error {
    /* Generic Failures */
    #[error("DB_FAILURE")]
    DbFailure(#[from] mongodb::error::Error),
    #[error("INVALID_OBJECT_ID")]
    InvalidObjectId(#[from] mongodb::bson::oid::Error),
    #[error("INVALID_DATE_TIME")]
    InvalidDateTime(#[from] mongodb::bson::datetime::Error),
    #[error("CHRONO_PARSE_FAILURE")]
    ChronoParseFailure(#[from] chrono::ParseError),

    /* Login Failure */
    #[error("COULD_NOT_LOGIN")]
    CouldNotLogIn,

    /* Signup failures */
    #[error("BRCYPT_FAILURE")]
    BcryptFailure(#[from] bcrypt::BcryptError),
    #[error("USER_ALREADY_EXISTS")]
    UserAlreadyExists,

    /* Auth failures */
    #[error("INVALID_SESSION_TOKEN")]
    InvalidSessionToken,
    #[error("SESSION_EXPIRED")]
    SessionExpired,
    #[error("SESSION_REVOKED")]
    SessionRevoked,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("FORBIDDEN")]
    Forbidden,

    /* General Not Found failures */
    #[error("USER_NOT_FOUND")]
    UserNotFound,
    #[error("REFRESH_TOKEN_NOT_FOUND")]
    RefreshTokenNotFound,
}

pub type Result<T = ()> = std::result::Result<T, Error>;

impl Error {
    pub fn get_status_code(&self) -> StatusCode {
        match self {
            Self::DbFailure(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidObjectId(_) => StatusCode::BAD_REQUEST,
            Self::InvalidDateTime(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ChronoParseFailure(_) => StatusCode::INTERNAL_SERVER_ERROR,

            Self::CouldNotLogIn => StatusCode::UNAUTHORIZED,

            Self::BcryptFailure(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::UserAlreadyExists => StatusCode::CONFLICT,

            Self::InvalidSessionToken => StatusCode::UNAUTHORIZED,
            Self::SessionExpired => StatusCode::UNAUTHORIZED,
            Self::SessionRevoked => StatusCode::UNAUTHORIZED,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,

            Self::UserNotFound => StatusCode::NOT_FOUND,
            Self::RefreshTokenNotFound => StatusCode::NOT_FOUND,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        // Do some logging
        match &self {
            Error::DbFailure(e) => {
                tracing::error!("A DB error occurred: {}", e);
            }
            Error::BcryptFailure(e) => {
                tracing::error!("A Bcrypt error occurred: {}", e);
            }
            Error::InvalidObjectId(e) => {
                tracing::error!("Got an invalid object id: {}", e);
            }
            Error::InvalidDateTime(e) => {
                tracing::error!("Tried to serialize an invalid DateTime: {}", e);
            }
            e => {
                tracing::error!("Request failed with Error: {error} {error:?}", error = e);
            }
        }

        let status_code = self.get_status_code();
        let body = Json(json!({
            "error": self.to_string()
        }));

        (status_code, body).into_response()
    }
}
