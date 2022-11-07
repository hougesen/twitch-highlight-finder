use std::sync::{Arc, Mutex};

use chat::socket_thread;
use dotenv::dotenv;
use parser::message_parser_thread;
use queue::Queue;
use tungstenite::Message;

mod chat;
mod parser;
mod queue;

type ArcRcMessageQueue = Arc<Mutex<Queue<Vec<(Message, u64)>>>>;

fn main() -> Result<(), tungstenite::Error> {
    println!("Starting Twitch Chat Collector");

    dotenv().ok();

    let channel_join_queue = Queue::from(vec!["esl_csgo".to_string(), "esl_csgo2".to_string()]);

    let message_queue: ArcRcMessageQueue = Arc::new(Mutex::new(Queue::new()));

    let message_queue_clone = Arc::clone(&message_queue);

    let _socket_thread_handle =
        std::thread::spawn(|| socket_thread(channel_join_queue, message_queue_clone));

    message_parser_thread(message_queue);

    Ok(())
}
