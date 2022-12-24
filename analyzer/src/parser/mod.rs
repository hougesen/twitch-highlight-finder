use mongodb::bson::DateTime;

#[derive(serde::Serialize)]
pub struct TwitchChatMessage {
    pub channel: String,
    pub sender: String,
    pub message: String,
    pub timestamp: DateTime,
}

/// :caveaio!caveaio@caveaio.tmi.twitch.tv PRIVMSG #hougesen :test
pub fn parse_message(msg: String, timestamp: DateTime) -> Option<TwitchChatMessage> {
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
    }

    None
}
