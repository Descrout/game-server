use std::sync::{Arc};
use tokio::sync::RwLock;
use std::collections::HashMap;
use crate::connection::Connection;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::stream::StreamExt;
use tungstenite::{Message, Result};
use crate::Events;
use futures_util::sink::SinkExt;

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

    pub async fn listen(lobby: Arc<RwLock<Self>>, mut receiver: UnboundedReceiver<Events>) {
        while let Some(event) = receiver.next().await {
            match event {
                Events::SetName(id, set_name) => {
                    {
                        let mut lobby = lobby.write().await;
                        lobby.connections.get_mut(&id).unwrap().name = Some(set_name.name);
                    }
                    Self::broadcast(lobby.clone(), vec![1,1,1,1]).await; // test
                },
                Events::Disconnect(id) => {
                    let mut lobby = lobby.write().await;
                    lobby.connections.remove(&id).unwrap();
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