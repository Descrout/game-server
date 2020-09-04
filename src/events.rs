use crate::connection::Connection;
use crate::ecs::game_state::GameState;
use crate::proto::proto_all::*;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::oneshot::Sender;

#[derive(Debug)]
pub enum LobbyEvents {
    Handshake(Sender<u32>, Connection),
    Disconnect(u32),
    CreateRoom(u32, Sender<UnboundedSender<GameEvents>>, CreateRoom),
    JoinRoom(u32, Sender<UnboundedSender<GameEvents>>, JoinRoom),
    TakeBack(Sender<()>, Connection),
    PlayerCount(u32, u32, usize),
    Chat(u32, Chat),
}

#[derive(Debug)]
pub enum GameEvents {
    Join(Connection),
    Quit(u32, Option<Sender<()>>),
    Chat(u32, Chat),
    Input(u32, GameInput),
    StateOut(Vec<GameState>),
}
