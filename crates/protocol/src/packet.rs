use anyhow::{anyhow, Result};
use uuid::Uuid;

#[derive(Debug)]
pub struct Packet<'a> {
    connection_id: Uuid,
    topic: u8,
    data: &'a [u8],
}

impl Packet<'_> {
    pub fn parse(connection_id: Uuid, buf: &[u8]) -> Result<Packet<'_>> {
        if connection_id.is_nil() {
            return Err(anyhow!("connection id cannot be nil"));
        }

        let topic = match buf.first() {
            Some(&topic) => topic,
            None => return Err(anyhow!("failed to parse topic")),
        };

        let data = &buf[1..];

        Ok(Packet {
            connection_id,
            topic,
            data,
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
        let buf = vec![];
        let actual = Packet::parse(Uuid::nil(), &buf).unwrap_err().to_string();
        assert_eq!(expected, actual);
    }

    #[test]
    fn unparsed_topic() {
        let expected = "failed to parse topic";
        let buf = vec![];
        let actual = Packet::parse(Uuid::new_v4(), &buf).unwrap_err().to_string();
        assert_eq!(expected, actual);
    }

    #[test]
    fn success_with_data() {
        let expected_uuid = Uuid::new_v4();
        let expected_topic = 42;
        let expected_data = vec![1, 2, 3];
        let buf = vec![42, 1, 2, 3];

        let packet = Packet::parse(expected_uuid, &buf).expect("failed to parse packet");

        assert_eq!(expected_uuid, packet.connection_id);
        assert_eq!(expected_topic, packet.topic);
        assert_eq!(expected_data, packet.data);
    }

    #[test]
    fn success_without_data() {
        let expected_uuid = Uuid::new_v4();
        let expected_topic = 42;
        let expected_data: Vec<u8> = vec![];
        let buf = vec![42];

        let packet = Packet::parse(expected_uuid, &buf).expect("failed to parse packet");

        assert_eq!(expected_uuid, packet.connection_id);
        assert_eq!(expected_topic, packet.topic);
        assert_eq!(expected_data, packet.data);
    }
}
