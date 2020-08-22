use tokio::net::{TcpStream};
use std::net::SocketAddr;
use tokio_tungstenite::WebSocketStream;
use futures_util::stream::{SplitSink, SplitStream};
use tungstenite::Message;
use tokio::sync::mpsc::UnboundedSender;
use futures_util::stream::StreamExt;
pub struct Connection {
    pub id: usize,
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

    pub async fn listen(id: usize, sender: UnboundedSender<String>, mut receiver: SplitStream<WebSocketStream<TcpStream>>) {
        let mut msg_future = receiver.next();
        println!("New connection listening {}", id);
        loop {
            
            match msg_future.await {
                Some(msg) => {
                    if let Ok(msg) = msg {
                        if msg.is_text() || msg.is_binary() {
                            println!("Message received : {}", msg);
                        }else {
                            println!("Connection lost {}", id);
                            break;
                        }
                    }else {
                        println!("Connection lost {}", id);
                        break;
                    }
                    msg_future = receiver.next();
                },
                None => {
                    println!("Connection lost {}", id);
                    break;
                },
            }
        }
    }
}