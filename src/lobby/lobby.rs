use std::collections::HashMap;
use crate::connection::Connection;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::stream::StreamExt;
use tungstenite::{Message};
use crate::Events;
use futures_util::sink::SinkExt;
use crate::proto::proto_all::*;
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

    //temporary poor mans regex
    fn incorrect_username(name: String) -> bool {
        if name.len() < 3 || name.len() > 20 {return true}
        for ch in name.chars() {
            if ch == '<' || ch == '>' || ch == '+' || 
            ch == '&' || ch == '%' || ch == '='{
                return true
            }
        }

        false
    }

    pub fn add_connection(&mut self, name: String) -> Result<u32, Error>{
        let id = if self.index_pool.len() > 0 {
            self.index_pool.pop().unwrap()
        }else if self.indices < self.max_client{
            self.indices += 1;
            self.indices
        }else {
            0
        };
        
        if id == 0 {
            return Err(Error{
                title: "Server is full".to_string(),
                message: "Server is currently full, try again later.".to_string()
            });
        }else if Self::incorrect_username(name){
            return Err(Error{
                title: "Username is not suitable.".to_string(),
                message: "Your username is not suitable, please try another one.".to_string()
            });
        }

        Ok(id)
    }

    fn serialize_error(err: Error) -> Vec<u8> {
        let mut out = Vec::new();
        let mut writer = Writer::new(&mut out);
        writer.write_message(&err).expect("Cannot serialize chat");
        out[0] = SendHeader::ERROR;
        out
    }

    fn serialize_chat(&self, id: u32, mut chat:  Chat) -> Vec<u8> {
        let conn = self.connections.get(&id).unwrap();

        chat.name = format!("({}) {}", id, conn.name);
        
        let mut out = Vec::new();
        let mut writer = Writer::new(&mut out);
        writer.write_message(&chat).expect("Cannot serialize chat");
        out[0] = SendHeader::CHAT;
        out
    }

    fn serialize_users(&self, my_id: u32) -> Vec<u8> {
        let mut u = Users{
            users: Vec::new(),
            me: my_id,
        };
        for (id, conn) in self.connections.iter() {
            u.users.push(User{id: *id, name : conn.name.clone()});
        }
        
        let mut out = Vec::new();
        let mut writer = Writer::new(&mut out);
        writer.write_message(&u).expect("Cannot serialize lobby");
        out[0] = SendHeader::USERS;
        out
    }

    pub async fn listen(mut receiver: UnboundedReceiver<Events>) {
        let mut lobby = Self::new();
        while let Some(event) = receiver.next().await {
            match event {
                Events::Handshake(tx, mut conn) => {
                    match lobby.add_connection(conn.name.clone()){
                        Ok(id) => {
                            conn.id = id;
                            lobby.connections.insert(id , conn);

                            if tx.send(id).is_ok(){
                                let data = lobby.serialize_users(id);
                                lobby.broadcast(data).await;
                            }else{
                                lobby.connections.remove(&id).unwrap();
                            }
                        },
                        Err(err) => {
                            let _ = conn.sender.send(Message::Binary(Self::serialize_error(err))).await;
                        }
                    }
                },
                Events::Disconnect(id) => {
                    let data = {
                        lobby.connections.remove(&id).unwrap();
                        lobby.serialize_users(id)
                    };
                    lobby.broadcast(data).await;
                    lobby.index_pool.push(id);
                    println!("Connection lost {}", id);
                },
                Events::CreateRoom(id, create_room) => {
    
                },
                Events::JoinRoom(id, join_room) => {
    
                },
                Events::Chat(id, chat) => {
                    let data = lobby.serialize_chat(id, chat);
                    lobby.broadcast(data).await;
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