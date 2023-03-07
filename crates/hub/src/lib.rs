use std::time::Duration;

use amqp::{
    consumer,
    publisher::{self, AmqpPublisher},
};
use anyhow::Result;
use log::{error, info, warn};
use protocol::packet::Packet;
use tokio::{
    io, join,
    net::{
        tcp::OwnedReadHalf,
        TcpListener, TcpStream,
    },
    time::timeout,
};
use uuid::Uuid;

pub struct HubConfig<'a> {
    pub address: &'a str,
    pub amqp_connection_string: &'a str,
}

pub async fn run(config: &HubConfig<'_>) -> Result<()> {
    let publisher = publisher::connect_publisher(config.amqp_connection_string)?;
    info!("connected amqp publisher");

    let _consumer = consumer::connect_consumer(config.amqp_connection_string, "hej")?;
    info!("connected amqp consumer");

    let listener = TcpListener::bind(config.address).await?;
    info!("listening on {}", config.address);

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                let p = publisher.clone();
                tokio::spawn(async move {
                    handle_stream(stream, p).await;
                });
            }
            Err(err) => {
                error!("failed to accept: {}", err);
            }
        }
    }
}

async fn handle_stream(stream: TcpStream, publisher: AmqpPublisher) {
    let id = Uuid::new_v4();
    info!("{} connected", id);
    let (reader, _writer) = stream.into_split();

    join!(handle_read(id, &reader, publisher),);

    info!("{} dropping", id);
}

async fn handle_read(id: Uuid, reader: &OwnedReadHalf, publisher: AmqpPublisher) {
    loop {
        // wait for the socket to be readable
        let duration = Duration::from_secs(5);
        let readable = reader.readable();
        let readable = timeout(duration, readable);

        if let Err(err) = readable.await {
            error!("{} failed to check readable: {}", id, err);
            break;
        }

        let mut buf = Vec::with_capacity(4096);

        // TODO: use iothub read implementation
        match reader.try_read_buf(&mut buf) {
            Ok(0) => {
                warn!("{} client disconnected", id);
                break;
            }
            Ok(_n) => {
                match Packet::from_raw(id, &buf) {
                    Ok(packet) => {
                        let msg = packet.serialize();

                        // TODO: remove .clone()
                        if let Err(err) = publisher.publish(msg.clone(), "hej") {
                            error!("{} failed to publish: {}", id, err);
                            break;
                        }

                        info!("{} published msg: {:?}", id, msg);
                    }
                    Err(err) => {
                        error!("{} sent corrupt data: {}", id, err);
                        break;
                    }
                }
            }
            Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => {
                warn!("{} would block", id);
                continue;
            }
            Err(err) => {
                error!("read failed for {}: {}", id, err);
                break;
            }
        }
    }
}
