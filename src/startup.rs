use crate::{configuration::Settings, health_check::health_check, product};
use axum::{routing::get, Router};
use mongodb::{Client, options::ClientOptions};


pub async fn run(configuration: Settings) -> anyhow::Result<()> {
    log::info!("Server is listening on http://127.0.0.1:8000");
    let db_client: Client = connect_database(configuration).await?;

    for db_name in db_client.database("test").list_collection_names(None).await? {
        log::info!("{:?}", db_name);
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

async fn connect_database(configuration: Settings) -> mongodb::error::Result<Client> {
    let client_options = ClientOptions::parse(configuration.database.connection_string()).await?;
    Client::with_options(client_options)
}
