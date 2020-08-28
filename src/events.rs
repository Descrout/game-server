use crate::proto::proto_all::*;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::oneshot::Sender;
use crate::connection::Connection;

#[derive(Debug)]
pub enum LobbyEvents{
    Handshake(Sender<u32>, Connection),
    Disconnect(u32),
    CreateRoom(u32, Sender<UnboundedSender<GameEvents>>, CreateRoom),
    JoinRoom(u32, Sender<UnboundedSender<GameEvents>>, JoinRoom),
    TakeBack(Sender<()>, Connection),
    PlayerCount(u32, usize),
    Chat(u32, Chat),
}

#[derive(Debug)]
pub enum GameEvents{
    Join(Connection),
    Quit(u32, Option<Sender<()>>),
    Chat(u32, Chat),
}