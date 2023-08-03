use axum::http::StatusCode;

pub async fn health_check() -> StatusCode {
    tracing::info!("/healthcheck is alive");
    StatusCode::OK
}
