use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub enum AppError {
    UnexpectedError(anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::UnexpectedError(e) => {
                log::error!("{}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "An error occured. Please try later.")
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> AppError {
        AppError::UnexpectedError(err.into())
    }
}
