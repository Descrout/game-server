use std::collections::HashMap;
use crate::connection::Connection;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tungstenite::{Message};
use crate::events::*;
use futures_util::sink::SinkExt;
use crate::proto::proto_all::*;
use quick_protobuf::{Writer};
use crate::headers::SendHeader;
use crate::ecs::game::Game;

pub struct Room{
    players: HashMap<u32, Connection>,
}

impl Room {
    pub fn new() -> Self{
        Self{
            players: HashMap::new(),
        }
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

    fn serialize_state(state: State) -> Vec<u8> {
        let mut out = Vec::new();
        let mut writer = Writer::new(&mut out);
        writer.write_message(&state).expect("Cannot serialize state");
        out[0] = SendHeader::STATE;
        out
    }

    pub async fn listen(game_id: u32, admin: Connection, mut receiver: UnboundedReceiver<GameEvents>, to_lobby: UnboundedSender<LobbyEvents>) {
        println!("New game listening : {} admin : {}", game_id, admin.id);
        let mut game = Game::new(admin.id);
        let (tx, rx) = mpsc::unbounded_channel::<GameEvents>();
        tokio::spawn(Self::broadcast(game_id, admin, rx, to_lobby));
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(17));
        loop {
            while let Ok(event) = receiver.try_recv(){
                match event {
                    GameEvents::Join(conn) => {
                        game.add_player(conn.id);
                        if tx.send(GameEvents::Join(conn)).is_err() {
                            return;
                        }
                    },
                    GameEvents::Quit(user_id, forward) => {
                        game.remove_player(&user_id);
                        if tx.send(GameEvents::Quit(user_id, forward)).is_err() {
                            return;
                        }
                    },
                    GameEvents::Input(id, input) => {
                        game.set_input(id, input);
                    },
                    _ => {
                        if tx.send(event).is_err() {
                            return;
                        }
                    }

                }
            }
            //game loop
            game.update();

            if tx.send(GameEvents::StateOut(game.state.clone())).is_err() {
                return;
            }
            interval.tick().await;
        }
    }

    async fn broadcast(room_id: u32, admin: Connection, mut rx: UnboundedReceiver<GameEvents>, to_lobby: UnboundedSender<LobbyEvents>) {
        let mut room = Self::new();
        room.players.insert(admin.id, admin);

        while let Some(event) = rx.recv().await {
            match event{
                GameEvents::Join(conn) => {
                    room.players.insert(conn.id, conn);
                },
                GameEvents::Quit(user_id, forward) => {
                    let conn = room.players.remove(&user_id).unwrap();
                    if let Some(f) = forward {
                        to_lobby.send(LobbyEvents::TakeBack(f, conn)).unwrap();
                    }
                    let len = room.players.len();
                    to_lobby.send(LobbyEvents::PlayerCount(room_id, user_id, len)).unwrap();
                    if len == 0 {
                        println!("Game terminated : {}", room_id);
                        rx.close();
                        return;
                    }
                },
                GameEvents::Chat(id, chat) => {
                    let data = room.serialize_chat(id, chat);
                    for conn in room.players.values_mut() {
                        let _ = conn.sender.send(Message::Binary(data.clone())).await;
                    }
                },
                GameEvents::StateOut(state) => {
                    let data = Self::serialize_state(state);
                    for conn in room.players.values_mut() {
                        let _ = conn.sender.send(Message::Binary(data.clone())).await;
                    }
                },
                _ => ()
            }
        }
    }
}