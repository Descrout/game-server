mod connection;
mod ecs;
mod events;
mod headers;
mod lobby;
mod proto;
mod room;

use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;

use connection::Connection;
use events::*;
use lobby::Lobby;
use tokio::sync::mpsc;

const PORT: &str = "6444";

#[tokio::main]
async fn main() {
    let addr = format!("0.0.0.0:{}", PORT);

    let mut listener = TcpListener::bind(&addr)
        .await
        .expect("Listening TCP failed.");

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
            }
        }
        //});
    }
}
