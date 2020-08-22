use std::sync::{Arc,RwLock};
use std::collections::HashMap;
use crate::connection::Connection;
use tokio::sync::mpsc::UnboundedReceiver;

pub struct Lobby{
    pub connections: Arc<RwLock<HashMap<usize, Connection>>>,
    pub indices: Arc<RwLock<usize>>,
    receiver: UnboundedReceiver<String>,
}

impl Lobby{
    pub fn new(receiver: UnboundedReceiver<String>) -> Self{
        Self{
            connections: Arc::new(RwLock::new(HashMap::new())),
            indices: Arc::new(RwLock::new(0usize)),
            receiver
        }
    }

    pub fn add_connection(&mut self, mut conn: Connection) -> usize{
        {
            let mut idx = self.indices.write().unwrap();
            *idx += 1;
        }
    
        let id = *self.indices.read().unwrap();

        conn.id = id;

        self.connections.write().unwrap().insert(id, conn);

        id
    }

    pub fn listen(&mut self) {
        println!("Listening lobby...");
    }
}