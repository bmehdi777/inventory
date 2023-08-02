use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::authentication::password::AuthenticationError;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Invalid credentials.")]
    AuthenticationError(#[from] AuthenticationError),
    #[error("Database error : {0}")]
    DatabaseError(#[from] mongodb::error::Error),
    #[error("Unexpected error : {0}")]
    UnexpectedError(#[from] anyhow::Error),
    #[error("Can't create a duplicated ressource.")]
    DuplicatedRessource,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::UnexpectedError(e) => {
                log::error!("{}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An error occured. Please try later.",
                )
            }
            AppError::DatabaseError(e) => {
                log::error!("Database error : {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An error occured. Please try later.",
                )
            }

            AppError::DuplicatedRessource => {
                log::error!("Duplicated ressource");
                (StatusCode::CONFLICT, "Can't create a duplicated ressource.")
            }
            AppError::AuthenticationError(e) => {
                log::error!("Authentication error : {}", e);
                (StatusCode::UNAUTHORIZED, "Invalid credentials.")
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

//impl<E> From<E> for AppError
//where
//    E: Into<anyhow::Error>,
//{
//    fn from(err: E) -> AppError {
//        AppError::UnexpectedError(err.into())
//    }
//}
//
