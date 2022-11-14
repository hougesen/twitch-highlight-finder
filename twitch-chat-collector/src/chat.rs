use crossbeam_channel::Sender;
use mongodb::bson::DateTime;
use tungstenite::{stream::MaybeTlsStream, Message, WebSocket};
use url::Url;

pub fn socket_thread(
    mut channel_queue: Vec<Message>,
    message_tx: Sender<(Message, DateTime)>,
) -> Result<(), tungstenite::Error> {
    let twitch_wss_uri = Url::parse("wss://irc-ws.chat.twitch.tv:443").unwrap();

    let (mut socket, _response) = tungstenite::connect(twitch_wss_uri)?;

    login_to_twitch(&mut socket)?;

    while !channel_queue.is_empty() {
        if let Some(channel) = channel_queue.pop() {
            socket.write_message(channel)?;
        }
    }

    drop(channel_queue);

    socket.write_pending()?;

    loop {
        if let Ok(message) = socket.read_message() {
            let timestamp = DateTime::now();

            if message.is_text() {
                // NOTE: no reason to waste time checking if succesful
                message_tx.try_send((message, timestamp));
            }
        }
    }
}

fn login_to_twitch(
    socket: &mut WebSocket<MaybeTlsStream<std::net::TcpStream>>,
) -> Result<(), tungstenite::Error> {
    let client_token = dotenv::var("CLIENT_TOKEN").expect("Missing env CLIENT_TOKEN");
    let client_username = dotenv::var("CLIENT_USERNAME").expect("Missing env CLIENT_USERNAME");

    socket.write_message(Message::Text(format!("PASS oauth:{}", &client_token)))?;

    socket.write_message(Message::Text(format!("NICK {}", &client_username)))?;

    println!("Sent login");

    Ok(())
}
