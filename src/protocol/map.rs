use serde_json::Value;
use std::fs::File;

use crate::protocol::packet::message::Message;

use super::{
    Type,
    packet::{character::Character, room::Room},
    send,
};

#[derive(Default, Debug, Clone)]
pub struct Map {
    pub rooms: Vec<Room>,
    pub players: Vec<Character>,
    pub monsters: Vec<Character>,
    pub desc_len: u16,
    pub desc: String,
}

impl Map {
    pub fn new() -> Self {
        Map {
            rooms: Vec::new(),
            players: Vec::new(),
            monsters: Vec::new(),
            desc_len: 0,
            desc: String::new(),
        }
    }

    pub fn find_room(&self, id: u16) -> Option<&Room> {
        self.rooms.iter().find(|room| room.room_number == id)
    }

    pub fn find_player(&self, name: String) -> Option<&Character> {
        self.players.iter().find(|player| player.name == name)
    }

    pub fn find_monster(&self, name: String) -> Option<&Character> {
        self.monsters.iter().find(|monster| monster.name == name)
    }

    pub fn add_player(&mut self, player: Character) {
        self.players.push(player);
    }

    pub fn add_monster(&mut self, monster: Character) {
        self.monsters.push(monster);
    }

    pub fn remove_player(&mut self, name: String) {
        if let Some(pos) = self.players.iter().position(|x| x.name == name) {
            self.players.remove(pos);
        }
    }

    pub fn remove_monster(&mut self, name: String) {
        if let Some(pos) = self.monsters.iter().position(|x| x.name == name) {
            self.monsters.remove(pos);
        }
    }

    /// Broadcast a message to all players in the game
    pub fn broadcast(&self, message: String) -> Result<(), std::io::Error> {
        println!("[BROADCAST] Sending message: {}", message);

        // Send the packet to the server
        for player in &self.players {
            send(Type::Message(Message {
                author: player.author.clone(),
                message_type: 1,
                message_len: message.len() as u16,
                recipient: player.name.clone(),
                sender: "Server".to_string(),
                narration: false,
                message: message.clone(),
            }))
            .unwrap_or_else(|e| {
                eprintln!(
                    "[BROADCAST] Failed to send message to {}: {}",
                    player.name, e
                );
            });
        }

        Ok(())
    }

    /// Alert all players in the current room of a character change
    pub fn alert() -> Result<(), std::io::Error> {
        Ok(())
    }

    pub fn build(data: File) -> Result<Self, serde_json::Error> {
        println!("[MAP] Building game map...");

        match serde_json::from_reader::<File, Value>(data) {
            Ok(json) => {
                let mut map = Map::new();

                // Parse the JSON data into the Map struct
                if let Some(tiles) = json["tiles"].as_array() {
                    // Add all existing room to the map
                    for tile in tiles {
                        let id = tile["id"].as_u64().unwrap_or(99) as u16;
                        let title = tile["title"].as_str().unwrap_or("ERROR").to_string();
                        let desc = tile["desc"].as_str().unwrap_or("ERROR").to_string();
                        let exits = tile["connections"]
                            .as_array()
                            .unwrap_or(&vec![])
                            .iter()
                            .filter_map(|v| v.as_u64())
                            .map(|v| v as usize)
                            .collect::<Vec<_>>();

                        let monsters = tile["monsters"]
                            .as_array()
                            .unwrap_or(&vec![])
                            .iter()
                            .filter_map(|v| v.as_u64())
                            .map(|v| v as usize)
                            .collect::<Vec<_>>();

                        // Create a new room and add it to the map
                        let room = Room::new(
                            id,
                            title.clone(),
                            exits.clone(),
                            monsters.clone(),
                            desc.clone(),
                        );

                        map.rooms.push(room.clone());

                        println!("[MAP] {:#?}", room);
                    }
                }

                //TODO: Compile all the monsters into the map's monster vector

                return Ok(map);
            }
            Err(e) => return Err(e),
        }
    }
}
