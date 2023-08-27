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

    #[error("Redis error : {0}")]
    RedisError(#[from] redis::RedisError),

    #[error("Can't create a duplicated ressource.")]
    DuplicatedRessource,

    #[error("Unable to parse base64.")]
    Base64Error(#[from] base64::DecodeError),

    #[error("Unable to reqwest.")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Image error.")]
    ImageError(#[from] image::ImageError),

    #[error("Can't lock ressource.")]
    LockError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::UnexpectedError(e) => {
                tracing::error!("{}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An error occured. Please try later.",
                )
            }
            AppError::ImageError(e) => {
                tracing::error!("Image error : {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An error occured with the image sent. Please try later.",
                )
            }
            AppError::DatabaseError(e) => {
                tracing::error!("Database error : {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An error occured. Please try later.",
                )
            }
            AppError::ReqwestError(e) => {
                tracing::error!("Reqwest error : {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An error occured while contacting OpenFoodFact. Please try later.",
                )
            }
            AppError::RedisError(e) => {
                tracing::error!("Redis error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An error occured. Please try later.",
                )
            }
            AppError::Base64Error(e) => {
                tracing::error!("Base64 error : {}.", e);
                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    "An error occured while decoding base64 data. Maybe the base64 payload is wrong."
                    )
            }
            AppError::DuplicatedRessource => {
                tracing::error!("Duplicated ressource");
                (StatusCode::CONFLICT, "Can't create a duplicated ressource.")
            }
            AppError::LockError => {
                tracing::error!("Unable to lock ressource");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An error occured. Please try later.",
                )
            }
            AppError::AuthenticationError(e) => {
                tracing::error!("Authentication error : {}", e);
                (StatusCode::UNAUTHORIZED, "Invalid credentials.")
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
