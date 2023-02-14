use uuid::Uuid;
use anyhow::{Result, anyhow};

#[derive(Debug)]
pub struct Packet {
    connection_id: Uuid,
    topic: u8,
    data: Vec<u8>,
}

impl Packet {
    pub fn parse(connection_id: Uuid, buf: Vec<u8>) -> Result<Packet> {
        let mut buf = buf;

        if connection_id.is_nil() {
            return Err(anyhow!("connection id cannot be nil"));
        }

        let topic = match buf.get(0) {
            Some(&topic) => {
                buf.remove(0);
                topic
            },
            None => return Err(anyhow!("failed to parse topic")),
        };

        Ok(Packet {
            connection_id,
            topic,
            data: buf,
        })
    }

    pub fn serialize(self) -> Vec<u8> {
        panic!(); // TODO: implement
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nil_connection_id() {
        let expected = "connection id cannot be nil";
        let actual = Packet::parse(Uuid::nil(), vec![]).unwrap_err().to_string();
        assert_eq!(expected, actual);
    }

    #[test]
    fn unparsed_topic() {
        let expected = "failed to parse topic";
        let actual = Packet::parse(Uuid::new_v4(), vec![]).unwrap_err().to_string();
        assert_eq!(expected, actual);
    }

    #[test]
    fn success() {
        let expected_uuid = Uuid::new_v4();
        let expected_topic = 42;
        let expected_data = vec![1,2,3];

        let packet = Packet::parse(expected_uuid, vec![42,1,2,3]).expect("failed to parse packet");

        assert_eq!(expected_uuid, packet.connection_id);
        assert_eq!(expected_topic, packet.topic);
        assert_eq!(expected_data, packet.data);
    }
}
