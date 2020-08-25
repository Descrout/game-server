use std::collections::HashMap;
use crate::connection::Connection;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::stream::StreamExt;
use tungstenite::{Message};
use crate::Events;
use futures_util::sink::SinkExt;
use crate::proto::proto_all;
use quick_protobuf::{Writer};
use crate::headers::SendHeader;

pub struct Lobby{
    pub connections: HashMap<u32, Connection>,
    pub indices: u32,
}

impl Lobby{
    pub fn new() -> Self{
        Self{
            connections: HashMap::new(),
            indices: 0,
        }
    }

    pub fn add_connection(&mut self, mut conn: Connection) -> u32{
        self.indices += 1;
        conn.id = self.indices;
        self.connections.insert(conn.id , conn);
        self.indices
    }

    fn serialize(&self) -> Vec<u8> {
        let mut lobby = proto_all::Lobby{
            users: Vec::new(),
            rooms: Vec::new(),
        };
        for (id, conn) in self.connections.iter() {
                lobby.users.push(proto_all::User{id: *id, name : conn.name.clone()});
        }
        
        let mut out = Vec::new();
        let mut writer = Writer::new(&mut out);
        writer.write_message(&lobby).expect("Cannot serialize lobby");
        out[0] = SendHeader::LOBBY;
        out
    }

    pub async fn listen(mut receiver: UnboundedReceiver<Events>) {
        let mut lobby = Self::new();
        while let Some(event) = receiver.next().await {
            match event {
                Events::Handshake(tx, conn) => {
                    let id = lobby.add_connection(conn);
                    if tx.send(id).is_ok(){
                        let data = lobby.serialize();
                        lobby.broadcast(data).await;
                    }else{
                        lobby.connections.remove(&id).unwrap();
                    }
                },
                Events::Disconnect(id) => {
                    let data = {
                        lobby.connections.remove(&id).unwrap();
                        lobby.serialize()
                    };
                    lobby.broadcast(data).await;
                    println!("Connection lost {}", id);
                },
                Events::CreateRoom(id, create_room) => {
    
                },
                Events::JoinRoom(id, join_room) => {
    
                }
            }
        }
    }


    async fn broadcast(&mut self, data: Vec<u8>) {
        for conn in self.connections.values_mut() {
            let _ = conn.sender.send(Message::Binary(data.clone())).await;
        }
    }
}