use std::io::Write;

use crate::debug_packet;
use crate::protocol::error::ErrorCode;
use crate::protocol::packet::{Packet, Parser};

#[derive(Default, Debug, Clone)]
pub struct Error {
    pub message_type: u8,
    pub error: ErrorCode,
    pub message_len: u16,
    pub message: String,
}

impl Error {
    pub fn new(error: ErrorCode, message: &str) -> Self {
        Error {
            message_type: 7,
            error,
            message_len: message.len() as u16,
            message: message.to_string(),
        }
    }
}

impl<'a> Parser<'a> for Error {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        // Package into a byte array
        let mut packet: Vec<u8> = Vec::new();

        packet.push(self.message_type);
        packet.push(self.error.clone().into());
        packet.extend(self.message_len.to_le_bytes());
        packet.extend(self.message.as_bytes());

        // Send the packet to the author
        writer.write_all(&packet).map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to write packet to buffer",
            )
        })?;

        debug_packet!(&packet);

        Ok(())
    }

    fn deserialize(packet: Packet) -> Result<Self, std::io::Error> {
        let message_type = packet.message_type;
        let error = ErrorCode::from(packet.body[0]);
        let message_len = u16::from_le_bytes([packet.body[1], packet.body[2]]);
        let message = String::from_utf8_lossy(&packet.body[3..])
            .trim_end_matches('\0')
            .to_string();
        
        Ok(Error {
            message_type,
            error,
            message_len,
            message,
        })
    }
}
