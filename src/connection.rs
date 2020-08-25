use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;
use futures_util::stream::{SplitSink, SplitStream};
use tungstenite::Message;
use tokio::sync::mpsc::UnboundedSender;
use futures_util::stream::StreamExt;
use crate::Events;
use quick_protobuf::{MessageRead, BytesReader, Result};
use crate::proto::proto_all::*;
use crate::headers::ReceiveHeader;
use tokio::sync::oneshot;

#[derive(Debug)]
pub struct Connection {
    pub id: u32,
    pub name: String,
    pub sender: SplitSink<WebSocketStream<TcpStream>, Message>,
}

impl Connection{
    pub fn new(name: String, sender: SplitSink<WebSocketStream<TcpStream>, Message>) -> Self {
        Self{
            id: 0,
            name: name,
            sender,
        }
    }

    pub fn parse_receive(id: u32, mut msg: Vec<u8>) -> Result<Events> {
        let header = msg.remove(0);
        let mut reader = BytesReader::from_bytes(&msg);
        match header{
            ReceiveHeader::CREATE_ROOM => Ok(Events::CreateRoom(id, CreateRoom::from_reader(&mut reader, &msg)?)),
            ReceiveHeader::CHAT => Ok(Events::Chat(id, Chat::from_reader(&mut reader, &msg)?)),
            _ => Err(quick_protobuf::Error::Message("Undefined header.".to_string())),
        }
    }

    pub async fn handshake(ws_stream: WebSocketStream<TcpStream>, to_lobby: UnboundedSender<Events>){
        let (sender, mut receiver) = ws_stream.split();
        let (tx, rx) = oneshot::channel::<u32>();

        while let Some(msg) = receiver.next().await {
            if let Ok(msg) = msg {
                if !msg.is_binary(){return}
                let mut msg = msg.into_data();
                let header = msg.remove(0);
                if header == ReceiveHeader::HANDSHAKE {
                    let mut reader = BytesReader::from_bytes(&msg);
                    if let Ok(hs) = Handshake::from_reader(&mut reader, &msg) {
                        let conn = Self::new(hs.name, sender);
                        to_lobby.send(Events::Handshake(tx, conn)).unwrap();
                        break;
                    }else {return}
                } else {return}
            }else {
                return;
            }
        }

        if let Ok(id) = rx.await {
            tokio::spawn(Self::listen(id, receiver, to_lobby));
        }else {
            println!("Handshake refused.");
        }
    }

    async fn listen(id: u32, mut receiver: SplitStream<WebSocketStream<TcpStream>>, to_lobby: UnboundedSender<Events>) {
        println!("Handshake accepted, ID[{}] listening...", id);
        while let Some(msg) = receiver.next().await {
            if let Ok(msg) = msg {
                if msg.is_binary() {
                    if let Ok(event) = Self::parse_receive(id, msg.into_data()){
                        to_lobby.send(event).unwrap();
                    }
                }else if msg.is_close(){
                    to_lobby.send(Events::Disconnect(id)).unwrap();
                    break;
                }
            }else {
                to_lobby.send(Events::Disconnect(id)).unwrap();
                break;
            }
        }
    }
}