use mongodb::bson::DateTime;

#[derive(serde::Serialize)]
pub struct ParsedMessage {
    pub channel: String,
    pub sender: String,
    pub message: String,
    pub timestamp: DateTime,
}

/// :caveaio!caveaio@caveaio.tmi.twitch.tv PRIVMSG #hougesen :test
pub fn parse_message(unparsed_message: String, timestamp: DateTime) -> Option<ParsedMessage> {
    if unparsed_message.contains("PRIVMSG") {
        let (sender, message) = unparsed_message.trim().split_once('!').unwrap();

        let sender = sender.replace(':', "");

        let (remaining, chat_message) = message.split_once(':').unwrap();

        let (_, channel) = remaining.split_once("PRIVMSG #").unwrap();

        return Some(ParsedMessage {
            channel: channel.trim().to_lowercase(),
            sender: sender.trim().to_lowercase(),
            timestamp,
            message: chat_message.trim().to_string(),
        });
    }

    None
}
