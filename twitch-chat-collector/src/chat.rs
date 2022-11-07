use crate::queue::Queue;

use std::net::TcpStream;
use tungstenite::{connect, stream::MaybeTlsStream, Message, WebSocket};
use url::Url;

pub fn socket_thread(mut channel_join_queue: Queue<String>) -> Result<(), tungstenite::Error> {
    let (mut socket, _response) =
        connect(Url::parse("wss://irc-ws.chat.twitch.tv:443").unwrap()).expect("Can't connect");

    login_to_twitch(&mut socket)?;

    loop {
        if !channel_join_queue.is_empty() {
            while !channel_join_queue.is_empty() {
                if let Some(channel) = channel_join_queue.dequeue() {
                    socket.write_message(
                        Message::Text(format!("JOIN #{}", &channel.to_lowercase())).into(),
                    )?;
                }
            }
        }

        let message = socket
            .read_message()
            .expect("Error reading message")
            .to_text()
            .unwrap()
            .to_owned();

        println!("Received: {}", message);

        if message.contains("PING") {
            socket.write_message(Message::Text(String::from("PONG")));
        }
    }
}

fn login_to_twitch(
    socket: &mut WebSocket<MaybeTlsStream<TcpStream>>,
) -> Result<(), tungstenite::Error> {
    let client_token = dotenv::var("CLIENT_TOKEN").expect("Missing env CLIENT_TOKEN");
    let client_username = dotenv::var("CLIENT_USERNAME").expect("Missing env CLIENT_USERNAME");

    socket.write_message(Message::Text(format!("PASS oauth:{}", &client_token)))?;

    socket.write_message(Message::Text(format!("NICK {}", &client_username)).into())?;

    Ok(())
}
