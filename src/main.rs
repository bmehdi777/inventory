use inventory_api::{startup, configuration};

#[tokio::main]
async fn main() {
    let configuration = configuration::get_configuration().expect("Failed to read env file");

    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );
    startup::run(configuration).await.unwrap();
}
