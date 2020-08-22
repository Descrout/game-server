mod connection;
mod lobby;

use tokio::net::{TcpListener};
use tokio_tungstenite::{accept_async};

use connection::Connection;
use std::sync::{Arc,RwLock};
use futures_util::{StreamExt};
use lobby::Lobby;
use tokio::sync::mpsc;


const PORT: &str = "6444";

#[tokio::main]
async fn main() {
    let addr = format!("127.0.0.1:{}", PORT);

    let mut listener = TcpListener::bind(&addr).await.expect("Listening TCP failed.");

    let (tx, rx) = mpsc::unbounded_channel::<String>();

    let lobby = Arc::new(RwLock::new(Lobby::new(rx)));

    // Listen lobby for room creation and chat
    let lobby_listen = lobby.clone();

    tokio::spawn( async move{
        lobby_listen.write().unwrap().listen();
    });

    println!("Listening on: {}", addr);

    while let Ok((stream, peer)) = listener.accept().await {
        let lobby_inner = lobby.clone();
        let tx = tx.clone();
        tokio::spawn(async move {
            match accept_async(stream).await {
                Err(e) => println!("Websocket connection error : {}", e),
                Ok(ws_stream) => {
                    let (sender, receiver) = ws_stream.split();
                    let conn = Connection::new(peer, sender);
                    let id = lobby_inner.write().unwrap().add_connection(conn);
                    tokio::spawn(Connection::listen(id , tx, receiver));
                },
            }
        });
    }
}