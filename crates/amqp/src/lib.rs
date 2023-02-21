use std::{
    sync::mpsc::{channel, Sender},
    thread,
};

use amiquip::{Connection, Exchange, Publish};
use anyhow::Result;

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

        while let Ok(msg) = rx.recv() {
            if let Err(err) = exchange.publish(Publish::new(&msg.1, &msg.0)) {
                println!("failed to publish message: {}", err);
                break;
            }
        }

        connection.close().expect("failed to close connection");
    });

    Ok(AmqpClient { tx })
}
