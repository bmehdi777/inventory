use inventory_api::{startup, configuration};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    let configuration = configuration::get_configuration().expect("Failed to read env file");

    startup::run(configuration).await.unwrap();
}
