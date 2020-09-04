use crate::events::*;
use crate::headers::ReceiveHeader;
use crate::proto::proto_all::*;
use futures_util::stream::StreamExt;
use futures_util::stream::{SplitSink, SplitStream};
use quick_protobuf::{BytesReader, MessageRead, Result};
use tokio::net::TcpStream;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::oneshot;
use tokio_tungstenite::WebSocketStream;
use tungstenite::Message;

#[derive(Debug)]
pub struct Connection {
    pub id: u32,
    pub name: String,
    pub sender: SplitSink<WebSocketStream<TcpStream>, Message>,
}

impl Connection {
    pub fn new(name: String, sender: SplitSink<WebSocketStream<TcpStream>, Message>) -> Self {
        Self {
            id: 0,
            name,
            sender,
        }
    }

    pub fn parse_receive_game(id: u32, header: u8, msg: Vec<u8>) -> Result<GameEvents> {
        let mut reader = BytesReader::from_bytes(&msg);
        match header {
            ReceiveHeader::GAME_CHAT if msg.len() < 105 => {
                Ok(GameEvents::Chat(id, Chat::from_reader(&mut reader, &msg)?))
            }
            ReceiveHeader::GAME_INPUT => Ok(GameEvents::Input(
                id,
                GameInput::from_reader(&mut reader, &msg)?,
            )),
            _ => Err(quick_protobuf::Error::Message(
                "Undefined header.".to_string(),
            )),
        }
    }

    pub fn parse_receive_lobby(
        id: u32,
        mut msg: Vec<u8>,
    ) -> Result<(
        Option<oneshot::Receiver<UnboundedSender<GameEvents>>>,
        LobbyEvents,
    )> {
        let header = msg.remove(0);
        let mut reader = BytesReader::from_bytes(&msg);
        match header {
            ReceiveHeader::CREATE_ROOM | ReceiveHeader::JOIN_ROOM => {
                let (tx, rx) = oneshot::channel::<UnboundedSender<GameEvents>>();
                if header == ReceiveHeader::CREATE_ROOM {
                    Ok((
                        Some(rx),
                        LobbyEvents::CreateRoom(
                            id,
                            tx,
                            CreateRoom::from_reader(&mut reader, &msg)?,
                        ),
                    ))
                } else {
                    Ok((
                        Some(rx),
                        LobbyEvents::JoinRoom(id, tx, JoinRoom::from_reader(&mut reader, &msg)?),
                    ))
                }
            }
            ReceiveHeader::LOBBY_CHAT if msg.len() < 105 => Ok((
                None,
                LobbyEvents::Chat(id, Chat::from_reader(&mut reader, &msg)?),
            )),
            _ => Err(quick_protobuf::Error::Message(
                "Undefined header.".to_string(),
            )),
        }
    }

    pub async fn handshake(
        ws_stream: WebSocketStream<TcpStream>,
        to_lobby: UnboundedSender<LobbyEvents>,
    ) {
        let (sender, mut receiver) = ws_stream.split();
        let (tx, rx) = oneshot::channel::<u32>();

        if let Some(msg) = receiver.next().await {
            if let Ok(msg) = msg {
                if !msg.is_binary() || msg.len() > 200 {
                    return;
                }
                let mut msg = msg.into_data();
                let header = msg.remove(0);
                if header == ReceiveHeader::HANDSHAKE {
                    let mut reader = BytesReader::from_bytes(&msg);
                    if let Ok(hs) = Handshake::from_reader(&mut reader, &msg) {
                        let conn = Self::new(hs.name, sender);
                        to_lobby.send(LobbyEvents::Handshake(tx, conn)).unwrap();
                    } else {
                        return;
                    }
                } else {
                    return;
                }
            } else {
                return;
            }
        }

        if let Ok(id) = rx.await {
            tokio::spawn(Self::listen_lobby(id, receiver, to_lobby));
        } else {
            println!("Handshake refused.");
        }
    }

    fn lobby_listener_spawner(
        id: u32,
        receiver: SplitStream<WebSocketStream<TcpStream>>,
        to_lobby: UnboundedSender<LobbyEvents>,
    ) {
        tokio::spawn(async move {
            Self::listen_lobby(id, receiver, to_lobby).await;
        });
    }

    async fn listen_game(
        id: u32,
        mut receiver: SplitStream<WebSocketStream<TcpStream>>,
        to_lobby: UnboundedSender<LobbyEvents>,
        to_game: UnboundedSender<GameEvents>,
    ) {
        while let Some(msg) = receiver.next().await {
            if let Ok(msg) = msg {
                if msg.is_binary() {
                    let mut msg = msg.into_data();
                    let header = msg.remove(0);
                    if let Ok(event) = Self::parse_receive_game(id, header, msg) {
                        to_game.send(event).unwrap();
                    } else if header == ReceiveHeader::QUIT_TO_LOBBY {
                        let (tx, rx) = oneshot::channel::<()>();
                        to_game.send(GameEvents::Quit(id, Some(tx))).unwrap();
                        if let Ok(()) = rx.await {
                            Self::lobby_listener_spawner(id, receiver, to_lobby);
                            return;
                        }
                    }
                } else if msg.is_close() {
                    break;
                }
            } else {
                break;
            }
        }
        to_game.send(GameEvents::Quit(id, None)).unwrap();
    }

    async fn listen_lobby(
        id: u32,
        mut receiver: SplitStream<WebSocketStream<TcpStream>>,
        to_lobby: UnboundedSender<LobbyEvents>,
    ) {
        println!("Handshake accepted, ID[{}] listening...", id);
        while let Some(msg) = receiver.next().await {
            if let Ok(msg) = msg {
                if msg.is_binary() {
                    if let Ok((recv, event)) = Self::parse_receive_lobby(id, msg.into_data()) {
                        to_lobby.send(event).unwrap();
                        if let Some(rx) = recv {
                            if let Ok(to_game) = rx.await {
                                tokio::spawn(Self::listen_game(id, receiver, to_lobby, to_game));
                                return;
                            }
                        }
                    }
                } else if msg.is_close() {
                    break;
                }
            } else {
                break;
            }
        }
        to_lobby.send(LobbyEvents::Disconnect(id)).unwrap();
    }
}
