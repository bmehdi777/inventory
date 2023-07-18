use crate::{configuration::Settings, health_check::health_check, product};
use axum::{routing::get, Router};

pub async fn run(configuration: Settings) {
    log::info!("Server is listening on http://127.0.0.1:8000");
    let app = Router::new()
        .route("/healthcheck", get(health_check))
        .route("/products", get(product::get_products));
    axum::Server::bind(
        &format!("127.0.0.1:{}", configuration.application_port)
            .parse()
            .unwrap(),
    )
    .serve(app.into_make_service())
    .await
    .unwrap();
}
