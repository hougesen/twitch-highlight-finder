use tungstenite::Message;

use crate::ArcRcMessageQueue;

#[derive(serde::Serialize)]
struct TwitchChatMessage {
    channel: String,
    sender: String,
    message: String,
    timestamp: u64,
}

pub fn message_parser_thread(message_queue: ArcRcMessageQueue) {
    loop {
        // Artificial delay so we don't end up blocking the socket_thread
        std::thread::sleep(std::time::Duration::from_secs(30));

        let mut message_queue_lock = message_queue.lock().unwrap();

        let message_buckets = message_queue_lock.dequeue_all();

        // Drop lock so we don't block socket_thread
        drop(message_queue_lock);

        if !message_buckets.is_empty() {
            let mut parsed_messages: Vec<TwitchChatMessage> = vec![];

            for bucket in message_buckets {
                for (message, timestamp) in bucket {
                    if let Some(parsed_message) = parse_message(message, timestamp) {
                        parsed_messages.push(parsed_message);
                    }
                }
            }

            save_messages(parsed_messages).ok();
        }
    }
}

/// :caveaio!caveaio@caveaio.tmi.twitch.tv PRIVMSG #hougesen :test
fn parse_message(socket_message: Message, timestamp: u64) -> Option<TwitchChatMessage> {
    let msg = socket_message.into_text().unwrap();

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

fn save_messages(messages: Vec<TwitchChatMessage>) -> Result<(), Box<dyn std::error::Error>> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let json = serde_json::to_string(&messages).unwrap();

    std::fs::write(format!("../dataset/messages_{timestamp}.json"), json)?;

    Ok(())
}
