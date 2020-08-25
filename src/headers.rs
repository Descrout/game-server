pub mod SendHeader {
    pub const LOBBY: u8 = 0;
    pub const CHAT: u8 = 1;
}

pub mod ReceiveHeader {
    pub const HANDSHAKE: u8 = 0;
    pub const CREATE_ROOM: u8 = 1;
    pub const JOIN_ROOM: u8 = 2;
    pub const CHAT: u8 = 3;
}