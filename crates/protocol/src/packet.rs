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
    pub fn from_raw(connection_id: Uuid, buf: &[u8]) -> Result<Packet<'_>> {
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

    pub fn to_raw(&self) -> Vec<u8> {
        let capacity = 2 + self.data.len();
        let mut raw: Vec<u8> = Vec::with_capacity(capacity);

        raw.push(self.device);
        raw.push(self.operation);

        for b in self.data {
            raw.push(*b);
        }

        raw
    }

    pub fn deserialize(buf: &[u8]) -> Result<Packet<'_>> {
        let connection_id = Uuid::new_v4();
        let packet = Packet::from_raw(connection_id, &buf[16..])?;
        Ok(packet)
    }

    pub fn serialize(self) -> Vec<u8> {
        let capacity = (128 / 8) + 2 + self.data.len();
        let mut serialized: Vec<u8> = Vec::with_capacity(capacity);

        for b in self.connection_id.as_bytes() {
            serialized.push(*b);
        }

        serialized.push(self.device);
        serialized.push(self.operation);

        for b in self.data {
            serialized.push(*b);
        }

        serialized
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_raw_nil_connection_id() {
        let expected = "connection id cannot be nil";
        let buf = vec![];

        let actual = Packet::from_raw(Uuid::nil(), &buf)
            .expect_err("expected err")
            .to_string();

        assert_eq!(expected, actual);
    }

    #[test]
    fn from_raw_unparsed_device() {
        let expected = "failed to parse device";
        let buf = vec![];

        let actual = Packet::from_raw(Uuid::new_v4(), &buf)
            .expect_err("expected err")
            .to_string();

        assert_eq!(expected, actual);
    }

    #[test]
    fn from_raw_unparsed_operation() {
        let expected = "failed to parse operation";
        let buf = vec![53];

        let actual = Packet::from_raw(Uuid::new_v4(), &buf)
            .expect_err("expected err")
            .to_string();

        assert_eq!(expected, actual);
    }

    #[test]
    fn from_raw_success_with_data() {
        let expected_uuid = Uuid::new_v4();
        let expected_device = 42;
        let expected_operation = 55;
        let expected_data = vec![1, 2, 3];
        let buf = vec![42, 55, 1, 2, 3];

        let packet = Packet::from_raw(expected_uuid, &buf).expect("failed to parse packet");

        assert_eq!(expected_uuid, packet.connection_id);
        assert_eq!(expected_device, packet.device);
        assert_eq!(expected_operation, packet.operation);
        assert_eq!(expected_data, packet.data);
    }

    #[test]
    fn from_raw_success_without_data() {
        let expected_uuid = Uuid::new_v4();
        let expected_device = 42;
        let expected_operation = 66;
        let expected_data: Vec<u8> = vec![];
        let buf = vec![42, 66];

        let packet = Packet::from_raw(expected_uuid, &buf).expect("failed to parse packet");

        assert_eq!(expected_uuid, packet.connection_id);
        assert_eq!(expected_device, packet.device);
        assert_eq!(expected_operation, packet.operation);
        assert_eq!(expected_data, packet.data);
    }

    #[test]
    fn serialize() {
        let id = [ 164, 221, 86, 77, 9, 34, 79, 96, 179, 2, 152, 246, 117, 254, 222, 146 ];
        let id = Uuid::from_bytes(id);
        let raw: Vec<u8> = vec![1,2,3];
        let expected = vec![164, 221, 86, 77, 9, 34, 79, 96, 179, 2, 152, 246, 117, 254, 222, 146, 1, 2, 3];

        let packet = Packet::from_raw(id, &raw).expect("failed to create packet");
        let serialized = packet.serialize();

        assert_eq!(serialized, expected);
    }

    #[test]
    fn deserialize() {
        let buf = vec![164, 221, 86, 77, 9, 34, 79, 96, 179, 2, 152, 246, 117, 254, 222, 146, 1, 2, 3];
        let expected_id = Uuid::from_bytes([164, 221, 86, 77, 9, 34, 79, 96, 179, 2, 152, 246, 117, 254, 222, 146]);

        let deserialized = Packet::deserialize(&buf).expect("failed to deserialize");

        assert_eq!(deserialized.connection_id, expected_id);
        assert_eq!(deserialized.device, 1);
        assert_eq!(deserialized.operation, 2);
        assert_eq!(deserialized.data, vec![3]);
    }

    #[test]
    fn deserialize_invalid_uuid() {
        todo!();
    }

    #[test]
    fn to_raw() {
        todo!();
    }
}
