use std::sync::mpsc::Receiver;
use tungstenite::Message;

#[derive(serde::Serialize)]
struct TwitchChatMessage {
    channel: String,
    sender: String,
    message: String,
    timestamp: u64,
}

pub fn message_parser_thread(message_rx: Receiver<(Message, u64)>) {
    let mut parsed_messages: Vec<TwitchChatMessage> = vec![];

    message_rx.iter().for_each(|(m, t)| {
        if let Some(parsed_message) = parse_message(m, t) {
            parsed_messages.push(parsed_message);
        }

        if parsed_messages.len() > 100 && save_messages(&parsed_messages).is_ok() {
            parsed_messages = vec![];
        }
    });
}

/// :caveaio!caveaio@caveaio.tmi.twitch.tv PRIVMSG #hougesen :test
fn parse_message(socket_message: Message, timestamp: u64) -> Option<TwitchChatMessage> {
    let msg = socket_message.into_text().unwrap();

    println!("message: {msg}");

    if msg.contains("PRIVMSG") {
        let (sender, message) = msg.split_once('!').unwrap();

        let sender = sender.replace(':', "");

        let (remaining, chat_message) = message.split_once(':').unwrap();

        let (_, channel) = remaining.split_once("PRIVMSG #").unwrap();

        return Some(TwitchChatMessage {
            channel: channel.trim().to_lowercase(),
            sender,
            timestamp,
            message: chat_message.trim().to_string(),
        });
    }

    None
}

fn save_messages(messages: &Vec<TwitchChatMessage>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Saving messages: {}", messages.len());

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let json = serde_json::to_string(messages).unwrap();

    std::fs::write(format!("../dataset/messages_{timestamp}.json"), json)?;

    Ok(())
}
