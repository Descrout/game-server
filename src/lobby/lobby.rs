use std::sync::{Arc};
use tokio::sync::RwLock;
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
            if let Some(name) = &conn.name {
                lobby.users.push(proto_all::User{id: *id, name : name.to_string()});
            }
        }
        
        let mut out = Vec::new();
        let mut writer = Writer::new(&mut out);
        writer.write_message(&lobby).expect("Cannot serialize lobby");
        out[0] = SendHeader::LOBBY;
        out
    }

    pub async fn listen(lobby: Arc<RwLock<Self>>, mut receiver: UnboundedReceiver<Events>) {
        while let Some(event) = receiver.next().await {
            match event {
                Events::SetName(id, set_name) => {
                    let data = {
                        let mut lobby = lobby.write().await;
                        lobby.connections.get_mut(&id).unwrap().name = Some(set_name.name);
                        lobby.serialize()
                    };
                    Self::broadcast(lobby.clone(), data).await;
                    println!("Name set ");
                },
                Events::Disconnect(id) => {
                    let data = {
                        let mut lobby = lobby.write().await;
                        lobby.connections.remove(&id).unwrap();
                        lobby.serialize()
                    };
                    Self::broadcast(lobby.clone(), data).await;
                    println!("Connection lost {}", id);
                },
                Events::CreateRoom(id, create_room) => {
    
                },
                Events::JoinRoom(id, join_room) => {
    
                }
            }
        }
    }


    async fn broadcast(lobby: Arc<RwLock<Self>>, data: Vec<u8>) {
        for conn in lobby.write().await.connections.values_mut() {
            let _ = conn.sender.send(Message::Binary(data.clone())).await;
        }
    }
}