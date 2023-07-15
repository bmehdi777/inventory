#[tokio::main]
async fn main() {
    let configuration = inventory_api::configuration::get_configuration().expect("Failed to read env file");

    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );
    inventory_api::startup::run(configuration).await;
}
