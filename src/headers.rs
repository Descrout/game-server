pub mod SendHeader {
    pub const USERS: u8 = 0;
    pub const ROOMS: u8 = 1;
    pub const LOBBY_CHAT: u8 = 2;
    pub const ERROR: u8 = 3;
    pub const GAME_CHAT: u8 = 4;
}

pub mod ReceiveHeader {
    pub const HANDSHAKE: u8 = 0;
    pub const CREATE_ROOM: u8 = 1;
    pub const JOIN_ROOM: u8 = 2;
    pub const LOBBY_CHAT: u8 = 3;
    pub const QUIT_TO_LOBBY: u8 = 4;
    pub const GAME_CHAT: u8 = 5;
}
