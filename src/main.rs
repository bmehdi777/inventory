use inventory_api::{configuration, startup};
use tracing_bunyan_formatter::BunyanFormattingLayer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

#[tokio::main]
async fn main() {
    set_tracing();

    let configuration = configuration::get_configuration().expect("Failed to read env file");

    startup::run(configuration).await.unwrap();
}

fn set_tracing() {
    let formated = BunyanFormattingLayer::new("Inventory_API".into(), std::io::stdout);
    let registry = tracing_subscriber::registry::Registry::default()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        //.with(tracing_subscriber::fmt::layer().compact());
        .with(formated);

    tracing::subscriber::set_global_default(registry).expect("Failed to set subscriber.");
}
