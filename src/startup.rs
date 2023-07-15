use axum::{routing::get, Router};
use crate::health_check;


pub async fn run() {
    log::info!("Server is listening on http://127.0.0.1:8000");
    let app = Router::new().route("/healthcheck", get(health_check));
    axum::Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
