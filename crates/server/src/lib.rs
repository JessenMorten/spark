use anyhow::Result;
use tokio::net::TcpListener;

pub struct ServerConfig<'a> {
    pub address: &'a str,
    pub amqp_connection_string: &'a str,
}

pub async fn run(config: &ServerConfig<'_>) -> Result<()> {
    let _amqp = amqp::connect(config.amqp_connection_string)?;
    let listener = TcpListener::bind(config.address).await?;
    println!("server running on {}", config.address);

    loop {
        match listener.accept().await {
            Ok((_stream, _)) => {
                println!("acception new socket");
                // TODO: spawn new action that executes a private function
                // which will read/write using the stream. Pass ownership
                // of the stream to the private function, since we don't need
                // it anymore.
            }
            Err(err) => {
                println!("failed to accept: {}", err);
            }
        }
    }
}
