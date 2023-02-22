use anyhow::Result;
use log::{debug, error, info};
use tokio::net::TcpListener;

pub struct HubConfig<'a> {
    pub address: &'a str,
    pub amqp_connection_string: &'a str,
}

pub async fn run(config: &HubConfig<'_>) -> Result<()> {
    let publisher = amqp::connect_publisher(config.amqp_connection_string)?;
    info!("connected amqp publisher");
    drop(publisher);

    let listener = TcpListener::bind(config.address).await?;
    info!("listening on {}", config.address);

    loop {
        match listener.accept().await {
            Ok((_stream, _)) => {
                debug!("acception new socket");
                // TODO: spawn new action that executes a private function
                // which will read/write using the stream. Pass ownership
                // of the stream to the private function, since we don't need
                // it anymore.
            }
            Err(err) => {
                error!("failed to accept: {}", err);
            }
        }
    }
}
