use crossbeam_channel::unbounded;
use dotenv::dotenv;
use tungstenite::Message;

use crate::chat::socket_thread;
use crate::parser::message_parser_thread;
use crate::queue::Queue;

mod chat;
mod db;
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

    let (message_tx, message_rx) = unbounded::<(Message, u64)>();

    std::thread::spawn(|| socket_thread(channel_join_queue, message_tx));

    message_parser_thread(message_rx);

    Ok(())
}
