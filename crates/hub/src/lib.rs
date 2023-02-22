use amqp::{consumer, publisher};
use anyhow::Result;
use log::{error, info};
use tokio::net::TcpListener;

pub struct HubConfig<'a> {
    pub address: &'a str,
    pub amqp_connection_string: &'a str,
}

pub async fn run(config: &HubConfig<'_>) -> Result<()> {
    let _publisher = publisher::connect_publisher(config.amqp_connection_string)?;
    info!("connected amqp publisher");

    let _consumer = consumer::connect_consumer(config.amqp_connection_string, "hej")?;
    info!("connected amqp consumer");

    let listener = TcpListener::bind(config.address).await?;
    info!("listening on {}", config.address);

    loop {
        match listener.accept().await {
            Ok((_stream, _)) => {
                info!("accepted new socket");
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
