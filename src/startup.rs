use crate::{
    configuration::Settings, health_check::health_check, middleware::authorize, product,
    routes::category, routes::login, routes::search, routes::user,
};
use axum::{middleware, routing::get, routing::post, routing::put, Router};
use hmac::{Hmac, Mac};
use mongodb::{options::ClientOptions, Client, Database};
use sha2::Sha256;
use std::sync::Arc;

#[derive(Debug)]
pub struct AppState {
    pub database: Database,
    pub jwt_secret: Hmac<Sha256>,
}
impl AppState {
    #[tracing::instrument]
    pub async fn new(configuration: &Settings) -> anyhow::Result<Self> {
        Ok(AppState {
            database: Self::connect_db(&configuration).await?,
            jwt_secret: Hmac::new_from_slice(
                configuration.application.jwt_secret.clone().as_bytes(),
            )?,
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
    tracing::info!(
        "Server is listening on http://{}:{}",
        &configuration.application.host,
        &configuration.application.port
    );

    let app_state = AppStateRC::new(AppState::new(&configuration).await?);

    let auth_route = Router::new()
        .route(
            "/products",
            get(product::get::get_products)
                .post(product::post::register_product)
                .delete(product::delete::delete),
        )
        .route(
            "/products/detail",
            post(product::get::get_product).put(product::put::update),
        )
        .route("/search/image", post(search::post::search_product_by_image))
        .route(
            "/search/barcode",
            post(search::post::search_product_by_barcode),
        )
        .route(
            "/category",
            get(category::get::get_categories)
                .post(category::post::create)
                .delete(category::delete::delete)
                .put(category::put::update),
        )
        .route("/users", get(user::get::get_users))
        .route("/user/personal_data", get(user::get::get_personal_info))
        .route("/user/edit/username", put(user::put::modify_username))
        .route("/user/edit/password", put(login::put::modify_password))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            authorize::block_without_valid_jwt,
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
