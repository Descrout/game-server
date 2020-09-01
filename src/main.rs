mod connection;
mod lobby;
mod proto;
mod headers;
mod events;
mod room;
mod ecs;

use tokio::net::{TcpListener};
use tokio_tungstenite::{accept_async};

use connection::Connection;
use lobby::Lobby;
use events::*;
use tokio::sync::mpsc;

const PORT: &str = "8081";
pub const DT: f32 = 0.016;

#[tokio::main]
async fn main() {
    let addr = format!("127.0.0.1:{}", PORT);

    let mut listener = TcpListener::bind(&addr).await.expect("Listening TCP failed.");

    let (lobby_sender, lobby_receiver) = mpsc::unbounded_channel::<LobbyEvents>();

    // Listen lobby for room creation and chat
    tokio::spawn(Lobby::listen(lobby_sender.clone(), lobby_receiver));
    
    println!("Listening on: {}", addr);
    
    // Accept new clients
    while let Ok((stream, peer)) = listener.accept().await {
        let lobby_sender = lobby_sender.clone();
        //tokio::spawn(async move {
        match accept_async(stream).await {
            Err(e) => println!("Websocket connection error : {}", e),
            Ok(ws_stream) => {
                println!("New connection : {}", peer);
                tokio::spawn(Connection::handshake(ws_stream, lobby_sender));
            },
        }
        //});
    }
}