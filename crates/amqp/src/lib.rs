use std::{
    sync::mpsc::{channel, Sender},
    thread,
};

use amiquip::{Connection, Exchange, Publish};
use anyhow::Result;
use log::{error, warn};

#[derive(Clone)]
pub struct AmqpClient {
    tx: Sender<(String, Vec<u8>)>,
}

impl AmqpClient {
    pub fn publish(self, data: Vec<u8>, queue: String) -> Result<()> {
        Ok(self.tx.send((queue, data))?)
    }
}

pub fn connect(url: &str) -> Result<AmqpClient> {
    let (tx, rx) = channel::<(String, Vec<u8>)>();
    let mut connection = Connection::insecure_open(url)?;
    let channel = connection.open_channel(None)?;

    thread::spawn(move || {
        let exchange = Exchange::direct(&channel);

        loop {
            match rx.recv() {
                Ok(msg) => {
                    let msg = Publish::new(&msg.1, &msg.0);
                    if let Err(err) = exchange.publish(msg) {
                        error!("failed to publish: {}", err);
                        break;
                    }
                },
                Err(err) => {
                    error!("failed to receive from rx: {}", err);
                    break;
                },
            }
        }

        match connection.close() {
            Ok(_) => warn!("closed amqp connection"),
            Err(_) => error!("failed to close amqp connection"),
        }
    });

    Ok(AmqpClient { tx })
}
