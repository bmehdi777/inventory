use crate::{configuration::Settings, health_check::health_check, product};
use axum::{routing::get, Router};
use mongodb::{options::ClientOptions, Client, Database};

pub async fn run(configuration: Settings) -> anyhow::Result<()> {
    log::info!("Server is listening on http://127.0.0.1:8000");

    let db_client = set_db(&configuration).await?;

    let app = Router::new()
        .route("/healthcheck", get(health_check))
        .route("/products", get(product::get_products))
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

pub async fn set_db(configuration: &Settings ) -> mongodb::error::Result<Database>  {
    let mut options = ClientOptions::parse(configuration.database.connection_string()).await?;
    options.connect_timeout = Some(std::time::Duration::new(5,0));
    options.direct_connection = Some(true);

    log::debug!("Connecting to database with the following options : {:?}", options);

    Ok(Client::with_options(options)?.database("inventory"))
}