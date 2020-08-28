use std::collections::HashMap;
use crate::connection::Connection;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::stream::StreamExt;
use tungstenite::{Message};
use crate::events::*;
use futures_util::sink::SinkExt;
use crate::proto::proto_all::*;
use quick_protobuf::{Writer};
use crate::headers::SendHeader;
use crate::game::Game;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::mpsc;

struct InfoRoom {
    r: Room,
    sender: UnboundedSender<GameEvents>,
}

pub struct Lobby{
    connections: HashMap<u32, Connection>,
    max_client: u32,
    connection_indices: u32,
    connection_index_pool: Vec<u32>,
    rooms: HashMap<u32, InfoRoom>,
    passwords: HashMap<u32, String>,
    room_indices: u32,
    room_index_pool: Vec<u32>,
}

impl Lobby{
    pub fn new() -> Self{
        Self{
            connections: HashMap::new(),
            max_client: 100,
            connection_indices: 0,
            connection_index_pool: Vec::new(),
            rooms: HashMap::new(),
            passwords: HashMap::new(),
            room_indices: 0,
            room_index_pool: Vec::new(),
        }
    }

    //temporary poor mans regex
    fn incorrect_name(name: String, len: usize) -> bool {
        if name.len() < 3 || name.len() > len {return true}
        for ch in name.chars() {
            if ch == '<' || ch == '>' || ch == '+' || 
            ch == '&' || ch == '%' || ch == '='{
                return true
            }
        }

        false
    }

    pub fn add_room(&mut self, name: String) -> Result<u32, Error>{
        let id = if self.room_index_pool.len() > 0 {
            self.room_index_pool.pop().unwrap()
        }else if self.room_indices < self.max_client{
            self.room_indices += 1;
            self.room_indices
        }else {
            0
        };
        
        if id == 0 {
            return Err(Error{
                title: "Max room count exceed!".to_string(),
                message: "Cannot create any more room, try again later.".to_string()
            });
        }else if Self::incorrect_name(name, 100){
            return Err(Error{
                title: "Room name is not suitable".to_string(),
                message: "Room name is not suitable, please try another one.".to_string()
            });
        }

        Ok(id)
    }

    fn join_to_room(&mut self, join_room: JoinRoom) -> Result<UnboundedSender<GameEvents>, Error> {
        if let Some(room) = self.rooms.get_mut(&join_room.id) {
            if let Some(pass) = self.passwords.get(&join_room.id) {
                if pass == &join_room.password {
                    if room.r.players < 6 {
                        room.r.players += 1;
                        return Ok(room.sender.clone());
                    }else {
                        return Err(Error{
                            title: "Room is full.".to_string(),
                            message: "Maximum player count reached.".to_string()
                        });
                    }
                }else {
                    return Err(Error{
                        title: "Incorrect room password.".to_string(),
                        message: "Incorrect password, try again.".to_string()
                    });
                }
            }else {
                if room.r.players < 6 {
                    room.r.players += 1;
                    return Ok(room.sender.clone());
                }else {
                    return Err(Error{
                        title: "Room is full.".to_string(),
                        message: "Maximum player count reached.".to_string()
                    });
                }
            }
        }else {
            return Err(Error{
                title: "Room cannot be find.".to_string(),
                message: "Room is no longer valid.".to_string()
            });
        }
    }

    pub fn add_connection(&mut self, name: String) -> Result<u32, Error>{
        let id = if self.connection_index_pool.len() > 0 {
            self.connection_index_pool.pop().unwrap()
        }else if self.connection_indices < self.max_client{
            self.connection_indices += 1;
            self.connection_indices
        }else {
            0
        };
        
        if id == 0 {
            return Err(Error{
                title: "Server is full".to_string(),
                message: "Server is currently full, try again later.".to_string()
            });
        }else if Self::incorrect_name(name, 20){
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
        out[0] = SendHeader::LOBBY_CHAT;
        out
    }

    fn serialize_users(users: Users) -> Vec<u8> {
        let mut out = Vec::new();
        let mut writer = Writer::new(&mut out);
        writer.write_message(&users).expect("Cannot serialize lobby");
        out[0] = SendHeader::USERS;
        out
    }

    fn serialize_rooms(&self) -> Vec<u8>  {
        let mut rooms = Rooms{
            rooms: Vec::new(),
        };

        for room in self.rooms.values() {
            rooms.rooms.push(room.r.clone());
        }

        let mut out = Vec::new();
        let mut writer = Writer::new(&mut out);
        writer.write_message(&rooms).expect("Cannot serialize lobby");
        out[0] = SendHeader::ROOMS;
        out
    }

    pub async fn listen(to_lobby: UnboundedSender<LobbyEvents>, mut receiver: UnboundedReceiver<LobbyEvents>) {
        let mut lobby = Self::new();
        while let Some(event) = receiver.next().await {
            match event {
                LobbyEvents::Handshake(tx, mut conn) => {
                    match lobby.add_connection(conn.name.clone()){
                        Ok(id) => {
                            conn.id = id;
                            lobby.connections.insert(id , conn);

                            if tx.send(id).is_ok(){
                                lobby.broadcast_lobby_info().await;
                            }else{
                                lobby.connections.remove(&id).unwrap();
                            }
                        },
                        Err(err) => {
                            let _ = conn.sender.send(Message::Binary(Self::serialize_error(err))).await;
                        }
                    }
                },
                LobbyEvents::PlayerCount(room_id, len) => {
                    if len == 0{
                        lobby.rooms.remove(&room_id).unwrap();
                        let _ = lobby.passwords.remove(&room_id);
                    }else{
                        lobby.rooms.get_mut(&room_id).unwrap().r.players = len as u32;
                    }
                },
                LobbyEvents::TakeBack(tx, conn) => {
                    lobby.connections.insert(conn.id , conn);
                    tx.send(()).unwrap();
                    lobby.broadcast_lobby_info().await;
                },
                LobbyEvents::Disconnect(id) => {
                    lobby.connections.remove(&id).unwrap();
                    lobby.connection_index_pool.push(id);
                    lobby.broadcast_users().await;
                    println!("Connection lost {}", id);
                },
                LobbyEvents::CreateRoom(user_id, tx, create_room) => {
                    match lobby.add_room(create_room.name.clone()){
                        Ok(room_id) => {
                            let password = create_room.password.len() > 0;
                            let room = Room{id: room_id, name: create_room.name, password, players: 1};

                            let (game_sender, game_receiver) = mpsc::unbounded_channel::<GameEvents>();

                            if tx.send(game_sender.clone()).is_err() {
                                continue;
                            }

                            if password {lobby.passwords.insert(room_id, create_room.password);}
                            lobby.rooms.insert(room_id , InfoRoom{r: room, sender: game_sender.clone()});

                            tokio::spawn(Game::listen(room_id, lobby.connections.remove(&user_id).unwrap(), game_receiver, to_lobby.clone()));

                            lobby.broadcast_lobby_info().await;
                        },
                        Err(err) => {
                            lobby.send_to(user_id, Self::serialize_error(err)).await;
                        }
                    }
                },
                LobbyEvents::JoinRoom(user_id, tx, join_room) => {
                    match lobby.join_to_room(join_room){
                        Ok(game_sender) => {
                            if tx.send(game_sender.clone()).is_err() {
                                continue;
                            }
                            game_sender.send(GameEvents::Join(lobby.connections.remove(&user_id).unwrap())).unwrap();
                        },
                        Err(err) => {
                            lobby.send_to(user_id, Self::serialize_error(err)).await;
                        }
                    }
                },
                LobbyEvents::Chat(id, chat) => {
                    let data = lobby.serialize_chat(id, chat);
                    lobby.broadcast(data).await;
                }
            }
        }
    }

    async fn broadcast_lobby_info(&mut self) {
        let mut users = Vec::new();
        for (id, conn) in self.connections.iter() {
            users.push(User{id: *id, name : conn.name.clone()});
        }
        let rooms = self.serialize_rooms();

        for conn in self.connections.values_mut() {
            let u = Users{users: users.clone(), me: conn.id};
            let _ = conn.sender.send(Message::Binary(Self::serialize_users(u))).await;
            let _ = conn.sender.send(Message::Binary(rooms.clone())).await;
        }
    }

    async fn broadcast_users(&mut self) {
        let mut users = Vec::new();
        for (id, conn) in self.connections.iter() {
            users.push(User{id: *id, name : conn.name.clone()});
        }
        for conn in self.connections.values_mut() {
            let u = Users{users: users.clone(), me: conn.id};
            let _ = conn.sender.send(Message::Binary(Self::serialize_users(u))).await;
        }
    }

    async fn send_to(&mut self, id: u32, data: Vec<u8>) {
        let _ = self.connections.get_mut(&id).unwrap().sender.send(Message::Binary(data)).await;       
    }

    async fn broadcast(&mut self, data: Vec<u8>) {
        for conn in self.connections.values_mut() {
            let _ = conn.sender.send(Message::Binary(data.clone())).await;
        }
    }
}