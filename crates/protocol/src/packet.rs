use anyhow::{anyhow, Result};
use uuid::Uuid;

#[derive(Debug)]
pub struct Packet<'a> {
    connection_id: Uuid,
    device: u8,
    operation: u8,
    data: &'a [u8],
}

impl Packet<'_> {
    pub fn parse(connection_id: Uuid, buf: &[u8]) -> Result<Packet<'_>> {
        if connection_id.is_nil() {
            return Err(anyhow!("connection id cannot be nil"));
        }

        let device = match buf.first() {
            Some(&device) => device,
            None => return Err(anyhow!("failed to parse device")),
        };

        let operation = match buf.get(1) {
            Some(&operation) => operation,
            None => return Err(anyhow!("failed to parse operation")),
        };

        let data = &buf[2..];

        Ok(Packet {
            connection_id,
            device,
            operation,
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

        let actual = Packet::parse(Uuid::nil(), &buf)
            .expect_err("expected err")
            .to_string();

        assert_eq!(expected, actual);
    }

    #[test]
    fn unparsed_device() {
        let expected = "failed to parse device";
        let buf = vec![];

        let actual = Packet::parse(Uuid::new_v4(), &buf)
            .expect_err("expected err")
            .to_string();

        assert_eq!(expected, actual);
    }

    #[test]
    fn unparsed_operation() {
        let expected = "failed to parse operation";
        let buf = vec![53];

        let actual = Packet::parse(Uuid::new_v4(), &buf)
            .expect_err("expected err")
            .to_string();

        assert_eq!(expected, actual);
    }

    #[test]
    fn success_with_data() {
        let expected_uuid = Uuid::new_v4();
        let expected_device = 42;
        let expected_operation = 55;
        let expected_data = vec![1, 2, 3];
        let buf = vec![42, 55, 1, 2, 3];

        let packet = Packet::parse(expected_uuid, &buf).expect("failed to parse packet");

        assert_eq!(expected_uuid, packet.connection_id);
        assert_eq!(expected_device, packet.device);
        assert_eq!(expected_operation, packet.operation);
        assert_eq!(expected_data, packet.data);
    }

    #[test]
    fn success_without_data() {
        let expected_uuid = Uuid::new_v4();
        let expected_device = 42;
        let expected_operation = 66;
        let expected_data: Vec<u8> = vec![];
        let buf = vec![42, 66];

        let packet = Packet::parse(expected_uuid, &buf).expect("failed to parse packet");

        assert_eq!(expected_uuid, packet.connection_id);
        assert_eq!(expected_device, packet.device);
        assert_eq!(expected_operation, packet.operation);
        assert_eq!(expected_data, packet.data);
    }
}
