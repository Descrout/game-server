use crate::proto::proto_all::*;

#[derive(Debug)]
pub enum Events{
    Handshake(tokio::sync::oneshot::Sender<u32>, crate::connection::Connection),
    Disconnect(u32),
    CreateRoom(u32, CreateRoom),
    JoinRoom(u32, JoinRoom),
    Chat(u32, Chat),
}