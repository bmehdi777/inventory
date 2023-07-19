use crate::{configuration::Settings, health_check::health_check, product};
use axum::{routing::get, Router};
use mongodb::{Client, options::ClientOptions};

pub async fn run(configuration: Settings) -> anyhow::Result<()> {
    log::info!("Server is listening on http://127.0.0.1:8000");

    let mut options = ClientOptions::parse(configuration.database.connection_string()).await?;
    options.connect_timeout = Some(std::time::Duration::new(1, 0));
    options.direct_connection = Some(true);

    log::info!("options : {:?}", options);

    let client = Client::with_options(options)?;

    for db_name in client.list_database_names(None, None).await? {
        log::info!("{}", db_name);
    }

    //let app = Router::new()
    //    .route("/healthcheck", get(health_check))
    //    .route("/products", get(product::get_products));
    //axum::Server::bind(
    //    &format!("127.0.0.1:{}", configuration.application_port)
    //        .parse()
    //        .unwrap(),
    //)
    //.serve(app.into_make_service())
    //.await
    //.unwrap();

    Ok(())
}

