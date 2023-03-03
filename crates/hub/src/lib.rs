use amqp::{consumer, publisher};
use anyhow::Result;
use log::{error, info};
use tokio::net::{TcpListener, TcpStream};

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
            Ok((stream, _)) => {
                tokio::spawn(async move {
                    handle_stream(stream).await;
                });
            }
            Err(err) => {
                error!("failed to accept: {}", err);
            }
        }
    }
}

async fn handle_stream(stream: TcpStream) {
    let addr = stream.peer_addr().expect("failed to get peer address");
    info!("{} connected", addr);
    info!("{} disconnected", addr);
}
