use crate::proto::proto_all::*;

#[derive(Debug)]
pub enum Events{
    SetName(u32, SetName),
    Disconnect(u32),
    CreateRoom(u32, CreateRoom),
    JoinRoom(u32, JoinRoom),
}