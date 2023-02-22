#[tokio::main()]
async fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let hub_config = hub::HubConfig {
        address: "127.0.0.1:8080",
        amqp_connection_string: "amqp://guest:guest@localhost:5672",
    };

    hub::run(&hub_config).await.expect("failed to run hub");
}
