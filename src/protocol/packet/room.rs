use std::io::Write;
use std::net::TcpStream;
use std::sync::Arc;

use crate::protocol::packet::{Packet, Parser};

#[derive(Default, Debug, Clone)]
pub struct Room {
    pub author: Option<Arc<TcpStream>>,
    pub message_type: u8,
    pub room_number: Vec<u8>, // Same as room_num in ChangeRoom
    pub room_name: Vec<u8>,
    pub description_len: u16,
    pub description: Vec<u8>,
}

impl<'a> Parser<'a> for Room {
    fn serialize<W: Write>(&self, _writer: &mut W) -> Result<(), std::io::Error> {
        // Implement serialization logic here
        Ok(())
    }
    fn deserialize(_packet: Packet) -> Result<Self, std::io::Error> {
        // Implement deserialization logic here
        Ok(Room::default())
    }
}