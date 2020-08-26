pub mod SendHeader {
    pub const USERS: u8 = 0;
    pub const ROOMS: u8 = 1;
    pub const CHAT: u8 = 2;
    pub const ERROR: u8 = 3;
}

pub mod ReceiveHeader {
    pub const HANDSHAKE: u8 = 0;
    pub const CREATE_ROOM: u8 = 1;
    pub const JOIN_ROOM: u8 = 2;
    pub const CHAT: u8 = 4;
}