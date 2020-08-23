use std::sync::{Arc,RwLock};
use std::collections::HashMap;
use crate::connection::Connection;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::stream::StreamExt;
use tungstenite::{Message, Result};
use crate::Events;

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
        while let Some(e) = receiver.next().await {
            lobby.write().unwrap().apply_event(e);
        }
    }

    fn apply_event(&mut self, event: Events) {
        match event {
            Events::SetName(id, set_name) => {
                self.connections.get_mut(&id).unwrap().name = Some(set_name.name);
            },
            Events::Disconnect(id) => {
                self.connections.remove(&id).unwrap();
                println!("Connection lost {}", id);
            },
            Events::CreateRoom(id, create_room) => {

            },
            Events::JoinRoom(id, join_room) => {

            }
        }
    }

    async fn broadcast(&mut self, data: Vec<u8>) -> Result<()>{
        use futures_util::sink::SinkExt;        
        for conn in self.connections.values_mut() {
            conn.sender.send(Message::Binary(data.clone())).await?;
        }
        Ok(())
    }
}