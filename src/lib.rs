use axum::{http::StatusCode, routing::get, Router};
use log::info;

pub async fn run() {
    let app = Router::new().route("/healthcheck", get(healthcheck));
    axum::Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn healthcheck() -> StatusCode {
    info!("/healthcheck is alive");
    StatusCode::OK
}
