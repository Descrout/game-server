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
    connections: HashMap<u32, Connection>,
    max_client: u32,
    indices: u32,
    index_pool: Vec<u32>,
}

impl Lobby{
    pub fn new() -> Self{
        Self{
            connections: HashMap::new(),
            max_client: 100,
            indices: 0,
            index_pool: Vec::new(),
        }
    }

    pub fn add_connection(&mut self, mut conn: Connection) -> Option<u32>{
        let id = if self.index_pool.len() > 0 {
            self.index_pool.pop().unwrap()
        }else if self.indices < self.max_client{
            self.indices += 1;
            self.indices
        }else {
            0
        };
        
        if id == 0 {return None}

        conn.id = id;
        self.connections.insert(id , conn);

        Some(id)
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
                    if let Some(id) = lobby.add_connection(conn){
                        if tx.send(id).is_ok(){
                            let data = lobby.serialize();
                            lobby.broadcast(data).await;
                        }else{
                            lobby.connections.remove(&id).unwrap();
                        }
                    }
                },
                Events::Disconnect(id) => {
                    let data = {
                        lobby.connections.remove(&id).unwrap();
                        lobby.serialize()
                    };
                    lobby.broadcast(data).await;
                    lobby.index_pool.push(id);
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