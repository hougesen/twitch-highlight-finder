use chat::socket_thread;
use dotenv::dotenv;
use parser::message_parser_thread;
use queue::Queue;
use std::sync::mpsc::channel;
use tungstenite::Message;

mod chat;
mod parser;
mod queue;

fn main() -> Result<(), tungstenite::Error> {
    println!("Starting Twitch Chat Collector");

    dotenv().ok();

    let channel_join_queue = Queue::from(vec![
        Message::Text("JOIN #esl_csgo".into()),
        Message::Text("JOIN #esl_csgo2".into()),
        Message::Text("JOIN #hougesen".into()),
    ]);

    let (message_tx, message_rx) = channel::<(Message, u64)>();

    std::thread::spawn(|| socket_thread(channel_join_queue, message_tx));

    message_parser_thread(message_rx);

    Ok(())
}
