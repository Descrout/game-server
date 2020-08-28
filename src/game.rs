use std::collections::HashMap;
use crate::connection::Connection;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tungstenite::{Message};
use crate::events::*;
use futures_util::sink::SinkExt;
use crate::proto::proto_all::*;
use quick_protobuf::{Writer};
use crate::headers::SendHeader;

pub struct Game{
    players: HashMap<u32, Connection>,
}

impl Game {
    pub fn new() -> Self{
        Self{
            players: HashMap::new(),
        }
    }

    fn serialize_error(err: Error) -> Vec<u8> {
        let mut out = Vec::new();
        let mut writer = Writer::new(&mut out);
        writer.write_message(&err).expect("Cannot serialize error");
        out[0] = SendHeader::ERROR;
        out
    }

    fn serialize_chat(&self, id: u32, mut chat:  Chat) -> Vec<u8> {
        let conn = self.players.get(&id).unwrap();

        chat.name = format!("({}) {}", id, conn.name);
        
        let mut out = Vec::new();
        let mut writer = Writer::new(&mut out);
        writer.write_message(&chat).expect("Cannot serialize chat");
        out[0] = SendHeader::GAME_CHAT;
        out
    }

    pub async fn listen(game_id: u32, admin: Connection, mut receiver: UnboundedReceiver<GameEvents>, to_lobby: UnboundedSender<LobbyEvents>) {
        let mut game = Self::new();
        game.players.insert(admin.id, admin);
        print!("New game listening.");
        loop{
            while let Ok(event) = receiver.try_recv(){
                match event{
                    GameEvents::Join(conn) => {
                        game.players.insert(conn.id, conn);
                    },
                    GameEvents::Quit(user_id, forward) => {
                        let conn = game.players.remove(&user_id).unwrap();
                        if let Some(f) = forward {
                            to_lobby.send(LobbyEvents::TakeBack(f, conn)).unwrap();
                        }
                        let len = game.players.len();
                        to_lobby.send(LobbyEvents::PlayerCount(game_id, len)).unwrap();
                        if len == 0{
                            return;
                        }
                    },
                    GameEvents::Chat(id, chat) => {
                        game.broadcast(game.serialize_chat(id, chat)).await;
                    }
                }
            }
        }
    }

    async fn broadcast(&mut self, data: Vec<u8>) {
        for conn in self.players.values_mut() {
            let _ = conn.sender.send(Message::Binary(data.clone())).await;
        }
    }
}