use axum::http::StatusCode;

pub async fn health_check() -> StatusCode {
    log::info!("/health_check is alive");
    StatusCode::OK
}
