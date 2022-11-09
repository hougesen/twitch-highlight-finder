use crossbeam_channel::Receiver;
use mongodb::{options::InsertManyOptions, sync::Database};
use tungstenite::Message;

#[derive(serde::Serialize)]
struct TwitchChatMessage {
    channel: String,
    sender: String,
    message: String,
    timestamp: u64,
}

pub fn message_parser_thread(db_client: Database, message_rx: Receiver<(Message, u64)>) -> ! {
    let mut parsed_messages: Vec<TwitchChatMessage> = Vec::new();

    let collection = db_client.collection::<TwitchChatMessage>("twitch_messages");

    let insert_options = InsertManyOptions::builder().ordered(Some(false)).build();

    message_rx.iter().for_each(|(m, t)| {
        if let Some(parsed_message) = parse_message(m, t) {
            parsed_messages.push(parsed_message);
        }

        if parsed_messages.len() > 100 {
            collection
                .insert_many(&parsed_messages, Some(insert_options.clone()))
                .ok();

            parsed_messages = Vec::new();
        }
    });

    panic!("Somehow out of message_parser_thread?");
}

/// :caveaio!caveaio@caveaio.tmi.twitch.tv PRIVMSG #hougesen :test
fn parse_message(socket_message: Message, timestamp: u64) -> Option<TwitchChatMessage> {
    let msg = socket_message
        .into_text()
        .unwrap_or_else(|_| "".to_string());

    if msg.contains("PRIVMSG") {
        let (sender, message) = msg.trim().split_once('!').unwrap();

        let sender = sender.replace(':', "");

        let (remaining, chat_message) = message.split_once(':').unwrap();

        let (_, channel) = remaining.split_once("PRIVMSG #").unwrap();

        return Some(TwitchChatMessage {
            channel: channel.trim().to_lowercase(),
            sender: sender.trim().to_lowercase(),
            timestamp,
            message: chat_message.trim().to_string(),
        });
    } else {
        println!("UNKNOWN MESSAGE: {:?}", msg.trim());
    }

    None
}
