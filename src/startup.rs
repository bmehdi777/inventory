use crate::{
    configuration::Settings, health_check::health_check, middleware::authorize, product,
    routes::login, routes::user, routes::search, session::SessionStore,
};
use axum::{middleware, routing::get, routing::post, Router};
use mongodb::{options::ClientOptions, Client, Database};
use std::sync::Arc;

#[derive(Debug)]
pub struct AppState {
    pub database: Database,
    pub session_store: SessionStore,
}
impl AppState {
    #[tracing::instrument]
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
            "Setting up connection information to database with the following options : {:?}",
            options
        );

        Ok(Client::with_options(options)?.database("inventory"))
    }
}
pub type AppStateRC = Arc<AppState>;

pub async fn run(configuration: Settings) -> anyhow::Result<()> {
    tracing::info!("Server is listening on http://127.0.0.1:8000");

    let app_state = AppStateRC::new(AppState::new(&configuration).await?);

    let auth_route = Router::new()
        .route("/products", get(product::get::get_products))
        .route("/products/register", post(product::post::register_product))
        .route("/search/image", post(search::post::search_product_by_image))
        .route("/search/barcode", post(search::post::search_product_by_barcode))
        .route("/users", get(user::get::get_users).put(user::put::modify_user))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            authorize::block_without_valid_cookie,
        ))
        .with_state(app_state.clone());

    let unauth_route = Router::new()
        .route("/healthcheck", get(health_check))
        .route("/auth/register", post(login::post::register))
        .route("/auth/login", post(login::post::login))
        .with_state(app_state.clone());

    let app = Router::new().merge(auth_route).merge(unauth_route);

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
