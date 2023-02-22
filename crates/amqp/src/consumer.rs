use std::{
    sync::mpsc::{channel, Receiver},
    thread,
};

use amiquip::{Connection, ConsumerMessage, ConsumerOptions, QueueDeclareOptions};
use anyhow::Result;
use log::{error, warn};

pub struct AmqpConsumer {
    rx: Receiver<Vec<u8>>,
}

impl AmqpConsumer {
    pub fn recv(&self) -> Result<Vec<u8>> {
        Ok(self.rx.recv()?)
    }
}

pub fn connect_consumer(url: &str, queue: &'static str) -> Result<AmqpConsumer> {
    let mut connection = Connection::insecure_open(url)?;
    let (tx, rx) = channel();
    let channel = connection.open_channel(None)?;

    thread::spawn(move || {
        let queue = match channel.queue_declare(queue, QueueDeclareOptions::default()) {
            Ok(queue) => queue,
            Err(err) => {
                error!("failed to declare queue: {}", err);
                return;
            }
        };

        let consumer = match queue.consume(ConsumerOptions::default()) {
            Ok(consumer) => consumer,
            Err(err) => {
                error!("failed to create consumer: {}", err);
                return;
            }
        };

        for (_, message) in consumer.receiver().iter().enumerate() {
            match message {
                ConsumerMessage::Delivery(delivery) => match tx.send(delivery.body.clone()) {
                    Ok(_) => {
                        if let Err(err) = consumer.ack(delivery) {
                            error!("failed to ack: {}", err);
                        }
                    }
                    Err(err) => error!("failed to send to tx: {}", err),
                },
                other => {
                    warn!("consumer ended: {:?}", other);
                    break;
                }
            }
        }

        match connection.close() {
            Ok(_) => warn!("closed amqp consumer connection"),
            Err(err) => error!("failed to close amqp consumer connection: {}", err),
        }
    });

    Ok(AmqpConsumer { rx })
}
