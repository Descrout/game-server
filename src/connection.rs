use tokio::net::{TcpStream};
use std::net::SocketAddr;
use tokio_tungstenite::WebSocketStream;
use futures_util::stream::{SplitSink, SplitStream};
use tungstenite::Message;
use tokio::sync::mpsc::UnboundedSender;
use futures_util::stream::StreamExt;
use crate::Events;
use quick_protobuf::{MessageRead, BytesReader, Result};
use crate::proto::proto_all::*;
use crate::headers::ReceiveHeader;
pub struct Connection {
    pub id: u32,
    pub name: Option<String>,
    pub peer: SocketAddr,
    pub sender: SplitSink<WebSocketStream<TcpStream>, Message>,
}

impl Connection{
    pub fn new(peer: SocketAddr, sender: SplitSink<WebSocketStream<TcpStream>, Message>) -> Self {
        Self{
            id: 0,
            name: None,
            peer,
            sender,
        }
    }

    pub fn parse_receive(id: u32, mut msg: Vec<u8>) -> Result<Events> {
        let header = msg.remove(0);
        let mut reader = BytesReader::from_bytes(&msg);
        match header{
            ReceiveHeader::SETNAME => Ok(Events::SetName(id, SetName::from_reader(&mut reader, &msg)?)),
            _ => Err(quick_protobuf::Error::Message("Undefined header.".to_string())),
        }
    }

    pub async fn listen(id: u32, to_lobby: UnboundedSender<Events>, mut receiver: SplitStream<WebSocketStream<TcpStream>>) {
        println!("New connection listening {}", id);
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