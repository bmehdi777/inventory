use crate::{
    configuration::Settings, health_check::health_check, product, routes::login, routes::user,
};
use axum::{routing::get, routing::post, Router};
use mongodb::{options::ClientOptions, Client, Database};
use std::sync::Arc;

pub type DatabaseRC = Arc<Database>;

pub async fn run(configuration: Settings) -> anyhow::Result<()> {
    log::info!("Server is listening on http://127.0.0.1:8000");

    let db_client = DatabaseRC::new(set_db(&configuration).await?);

    let app = Router::new()
        .route("/healthcheck", get(health_check))
        .route("/products", get(product::get::get_products))
        .route("/users", get(user::get::get_users))
        .route("/auth/register", post(login::post::register))
        .route("/auth/login", post(login::post::login))
        .with_state(db_client.clone());

    axum::Server::bind(
        &format!("127.0.0.1:{}", configuration.application_port)
            .parse()
            .unwrap(),
    )
    .serve(app.into_make_service())
    .await
    .unwrap();

    Ok(())
}

pub async fn set_db(configuration: &Settings) -> mongodb::error::Result<Database> {
    let mut options = ClientOptions::parse(configuration.database.connection_string()).await?;
    options.connect_timeout = Some(std::time::Duration::new(5, 0));
    options.direct_connection = Some(true);

    log::debug!(
        "Connecting to database with the following options : {:?}",
        options
    );

    Ok(Client::with_options(options)?.database("inventory"))
}
