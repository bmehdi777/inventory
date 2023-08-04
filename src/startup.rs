use crate::{
    configuration::Settings, health_check::health_check, product, routes::login, routes::user,
    session::SessionStore,
};
use axum::{routing::get, routing::post, Router};
use mongodb::{options::ClientOptions, Client, Database};
use std::sync::Arc;

pub struct AppState {
    pub database: Database,
    pub session_store: SessionStore,
}
impl AppState {
    pub async fn new(configuration: &Settings) -> anyhow::Result<Self> {
        Ok(AppState {
            database: Self::connect_db(&configuration).await?,
            session_store: SessionStore::new(configuration.redis_uri.clone()).await?,
        })
    }
    #[tracing::instrument]
    async fn connect_db(configuration: &Settings) -> mongodb::error::Result<Database> {
        let mut options = ClientOptions::parse(configuration.database.connection_string()).await?;
        options.connect_timeout = Some(std::time::Duration::new(5, 0));
        options.direct_connection = Some(true);

        tracing::info!(
            "Connecting to database with the following options : {:?}",
            options
            );

        Ok(Client::with_options(options)?.database("inventory"))
    }
}
pub type AppStateRC = Arc<AppState>;

pub async fn run(configuration: Settings) -> anyhow::Result<()> {
    tracing::info!("Server is listening on http://127.0.0.1:8000");

    let app_state = AppStateRC::new(AppState::new(&configuration).await?);

    let app = Router::new()
        .route("/healthcheck", get(health_check))
        .route("/products", get(product::get::get_products))
        .route("/users", get(user::get::get_users))
        .route("/auth/register", post(login::post::register))
        .route("/auth/login", post(login::post::login))
        .with_state(app_state.clone());

    axum::Server::bind(
        &format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        )
        .parse()
        .unwrap(),
    )
    .serve(app.into_make_service())
    .await
    .unwrap();

    Ok(())
}
