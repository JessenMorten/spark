#[tokio::main()]
async fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let server_config = server::ServerConfig {
        address: "127.0.0.1:8080",
        amqp_connection_string: "amqp://guest:guest@localhost:5672",
    };

    server::run(&server_config)
        .await
        .expect("failed to run server");
}
