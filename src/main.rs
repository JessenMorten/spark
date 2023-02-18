#[tokio::main()]
async fn main() {
    let server_config = server::ServerConfig {
        address: "127.0.0.1:8080",
    };

    server::run(&server_config)
        .await
        .expect("failed to run server");
}
