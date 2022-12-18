use async_channel::Sender;
use mongodb::bson::DateTime;
use tungstenite::{http::Uri, stream::MaybeTlsStream, Message, WebSocket};

fn connect_to_twitch_wss(
) -> Result<WebSocket<MaybeTlsStream<std::net::TcpStream>>, tungstenite::Error> {
    let (mut socket, _response) =
        tungstenite::connect("wss://irc-ws.chat.twitch.tv:443".parse::<Uri>().unwrap())?;

    login_to_twitch(&mut socket)?;

    Ok(socket)
}

pub async fn socket_thread(
    channel_queue: Vec<Message>,
    message_tx: Sender<(String, DateTime)>,
) -> Result<(), tungstenite::Error> {
    let mut socket = connect_to_twitch_wss()?;

    join_channels(&mut socket, &channel_queue);
    socket.write_pending()?;

    loop {
        match socket.read_message() {
            Ok(message) => {
                let timestamp = DateTime::now();

                if message.is_text() {
                    let message_text = message.into_text().unwrap_or_else(|_| "".to_string());

                    if message_text.contains("PING") {
                        println!("message contains ping");
                        socket.write_message(Message::Text("PONG".to_string())).ok();
                        socket.write_pending().ok();
                    } else {
                        // NOTE: no reason to waste time checking if succesful
                        message_tx.send((message_text, timestamp)).await;
                    }
                }
            }
            Err(error) => match error {
                tungstenite::Error::ConnectionClosed => {
                    println!("tungstenite::Error::ConnectionClosed error {}", error);

                    socket = connect_to_twitch_wss()?;

                    println!("Done reconnecting");

                    join_channels(&mut socket, &channel_queue);
                    socket.write_pending()?;
                }
                // tungstenite::Error::AlreadyClosed => todo!(),
                // tungstenite::Error::Io(_) => todo!(),
                // tungstenite::Error::Tls(_) => todo!(),
                // tungstenite::Error::Capacity(_) => todo!(),
                // tungstenite::Error::Protocol(_) => todo!(),
                // tungstenite::Error::SendQueueFull(_) => todo!(),
                // tungstenite::Error::Utf8 => todo!(),
                // tungstenite::Error::Url(_) => todo!(),
                // tungstenite::Error::Http(_) => todo!(),
                // tungstenite::Error::HttpFormat(_) => todo!(),
                _ => println!("socket-read_message error {}", error),
            },
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

fn join_channels(
    socket: &mut WebSocket<MaybeTlsStream<std::net::TcpStream>>,
    channels: &Vec<Message>,
) {
    for channel in channels {
        socket.write_message(channel.to_owned()).ok();
    }
}
