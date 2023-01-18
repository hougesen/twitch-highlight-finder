use async_channel::Sender;
use mongodb::bson::DateTime;
use queue::Queue;
use tungstenite::{http::Uri, stream::MaybeTlsStream, Message, WebSocket};

use crate::queue::channel::{check_queue, Event};

fn connect_to_twitch_wss(
) -> Result<WebSocket<MaybeTlsStream<std::net::TcpStream>>, tungstenite::Error> {
    let (mut socket, _response) =
        tungstenite::connect("wss://irc-ws.chat.twitch.tv:443".parse::<Uri>().unwrap())?;

    login_to_twitch(&mut socket)?;

    Ok(socket)
}

#[allow(unused_must_use)]
pub async fn chat_listener(
    message_tx: Sender<(String, DateTime)>,
) -> Result<(), tungstenite::Error> {
    let mut event_queue = Queue::new(None).await;

    let create_queue_output = event_queue.create_queue("live-tracker").await;

    event_queue.set_queue_url(create_queue_output.unwrap().queue_url().unwrap());

    let mut joined_channels: std::collections::HashSet<String> = std::collections::HashSet::new();

    let mut socket = connect_to_twitch_wss()?;

    join_channels(
        &mut socket,
        &mut joined_channels,
        check_queue(&event_queue).await,
    );

    socket.write_pending()?;

    let mut last_channel_queue_check = std::time::Instant::now();

    while !message_tx.is_closed() {
        if last_channel_queue_check.elapsed().as_secs() > 30 {
            join_channels(
                &mut socket,
                &mut joined_channels,
                check_queue(&event_queue).await,
            );

            last_channel_queue_check = std::time::Instant::now()
        }

        match socket.read_message() {
            Ok(message) => {
                let timestamp = DateTime::now();

                if message.is_text() {
                    let message_text = message.into_text().unwrap_or_else(|_| "".to_string());

                    if message_text.contains("PING") {
                        println!("message contains ping");
                        socket.write_message(Message::Text("PONG".to_string()));
                        socket.write_pending();
                    } else {
                        message_tx.send((message_text, timestamp)).await;
                    }
                }
            }
            Err(error) => match error {
                tungstenite::Error::ConnectionClosed => {
                    println!("tungstenite::Error::ConnectionClosed error {}", error);

                    socket = connect_to_twitch_wss()?;

                    println!("Done reconnecting");

                    join_channels(
                        &mut socket,
                        &mut joined_channels,
                        check_queue(&event_queue).await,
                    );

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

    eprintln!("outside of chat_listener loop");

    Ok(())
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

#[inline]
fn construct_join_message(channel: &str) -> Message {
    Message::Text(format!("JOIN #{}", channel.to_lowercase().trim()))
}

#[inline]
fn construct_leave_message(channel: &str) -> Message {
    Message::Text(format!("PART #{}", channel.to_lowercase().trim()))
}

#[allow(unused_must_use)]
fn join_channels(
    socket: &mut WebSocket<MaybeTlsStream<std::net::TcpStream>>,
    joined_channels: &mut std::collections::HashSet<String>,
    events: Vec<Event>,
) {
    if events.is_empty() {
        for channel in joined_channels.iter() {
            socket.write_message(construct_join_message(channel));
        }
    }

    for event in events {
        if event.kind == "stream.online" {
            socket.write_message(construct_join_message(&event.username));
            joined_channels.insert(event.username);
        } else {
            joined_channels.remove(&event.username);
            socket.write_message(construct_leave_message(&event.username));
        }
    }
}
