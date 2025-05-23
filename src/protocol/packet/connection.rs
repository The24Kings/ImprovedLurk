use std::io::Write;

use crate::{debug_packet, protocol::packet::{Packet, Parser}};

use super::room::Room;

#[derive(Default, Debug, Clone)]
pub struct Connection {
    pub message_type: u8,
    pub room_number: u16,
    pub room_name: String,
    pub description_len: u16,
    pub description: String,
}

impl Connection {
    /// Create a new connection from the game map to send to the client
    pub fn from(room: &Room) -> Self {
        Connection {
            message_type: 13,
            room_number: room.room_number,
            room_name: room.room_name.clone(),
            description_len: room.desc_short.len() as u16,
            description: room.desc_short.clone()
        }
    }
}

impl<'a> Parser<'a> for Connection {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        // Package into a byte array
        let mut packet: Vec<u8> = Vec::new();

        packet.push(self.message_type);
        packet.extend(self.room_number.to_le_bytes());

        let mut room_name_bytes = self.room_name.as_bytes().to_vec();
        room_name_bytes.resize(32, 0x00); // Pad the name to 32 bytes
        packet.extend(room_name_bytes);

        packet.extend(self.description_len.to_le_bytes());
        packet.extend(self.description.as_bytes());

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
        let room_number = u16::from_le_bytes([packet.body[0], packet.body[1]]);
        let room_name = String::from_utf8_lossy(&packet.body[2..34])
            .trim_end_matches('\0')
            .to_string();
        let description_len = u16::from_le_bytes([packet.body[34], packet.body[35]]);
        let description = String::from_utf8_lossy(&packet.body[36..]).to_string();

        Ok(Connection {
            message_type,
            room_number,
            room_name,
            description_len,
            description,
        })
    }
}
