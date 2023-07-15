use axum::http::StatusCode;

pub async fn health_check() -> StatusCode {
    log::info!("/healthcheck is alive");
    StatusCode::OK
}
