use std::net::TcpStream;
use tungstenite::{connect, stream::MaybeTlsStream, Message, WebSocket};
use url::Url;

use crate::{queue::Queue, ArcRcMessageQueue};

pub fn socket_thread(
    mut channel_join_queue: Queue<String>,
    message_queue: ArcRcMessageQueue,
) -> Result<(), tungstenite::Error> {
    let (mut socket, _response) =
        connect(Url::parse("wss://irc-ws.chat.twitch.tv:443").unwrap()).expect("Can't connect");

    login_to_twitch(&mut socket)?;

    let mut unqueued_messages: Vec<(Message, u64)> = vec![];

    loop {
        if !channel_join_queue.is_empty() {
            while !channel_join_queue.is_empty() {
                if let Some(channel) = channel_join_queue.dequeue() {
                    socket.write_message(Message::Text(format!(
                        "JOIN #{}",
                        &channel.trim().to_lowercase()
                    )))?;
                }
            }
        }

        if let Ok(message) = socket.read_message() {
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();

            if message.is_text() {
                unqueued_messages.push((message, timestamp));
            }
        }

        if unqueued_messages.len() > 1000 {
            let mut message_queue_lock = message_queue.lock().unwrap();

            message_queue_lock.enqueue(unqueued_messages);

            drop(message_queue_lock);

            unqueued_messages = vec![];
        }
    }
}

fn login_to_twitch(
    socket: &mut WebSocket<MaybeTlsStream<TcpStream>>,
) -> Result<(), tungstenite::Error> {
    let client_token = dotenv::var("CLIENT_TOKEN").expect("Missing env CLIENT_TOKEN");
    let client_username = dotenv::var("CLIENT_USERNAME").expect("Missing env CLIENT_USERNAME");

    socket.write_message(Message::Text(format!("PASS oauth:{}", &client_token)))?;

    socket.write_message(Message::Text(format!("NICK {}", &client_username)))?;

    Ok(())
}
