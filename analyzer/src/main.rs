use queue::Queue;

use crate::db::{
    emotes::get_emote_scores,
    get_db_client,
    messages::{save_message_batch, TwitchChatMessage},
};

mod analysis;
mod db;
mod parser;

#[derive(Debug, serde::Deserialize)]
pub struct QueueMessage {
    pub message: String,
    pub timestamp: mongodb::bson::DateTime,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_client = get_db_client().await.unwrap();

    let emote_scores = std::sync::Arc::new(
        get_emote_scores(&db_client.database("highlights"))
            .await
            .into_read_only(),
    );

    if emote_scores.is_empty() {
        panic!("Emote score is empty!")
    }

    let mut threads = Vec::new();

    for id in 0..std::thread::available_parallelism().map_or(1, usize::from) {
        let emote_scores_ref = std::sync::Arc::clone(&emote_scores);

        let db_client_ref = db_client.clone();

        let handle = tokio::spawn(job(id, db_client_ref, emote_scores_ref));

        threads.push(handle);
    }

    for thread in threads {
        thread.await.ok();
    }

    println!("queue is empty now");

    Ok(())
}

async fn job(
    id: usize,
    db_client: mongodb::Client,
    emote_scores: std::sync::Arc<dashmap::ReadOnlyView<String, u8>>,
) -> Result<(), mongodb::error::Error> {
    println!("job: {id}");

    let database = db_client.database("highlights");

    let mut queue = Queue::new(None).await;

    let created_queue = queue.create_queue("unparsed-messages").await.unwrap();

    queue.set_queue_url(created_queue.queue_url().unwrap());

    loop {
        let mut finished_messages: Vec<TwitchChatMessage> = Vec::with_capacity(10);
        let mut receipt_handles = Vec::with_capacity(10);

        if let Ok(queue_messages) = queue
            .get_message_batch::<QueueMessage>(Some(10), Some(false))
            .await
        {
            for (queue_message, message_handle) in queue_messages {
                if let Some(parsed_message) =
                    parser::parse_message(&queue_message.message, queue_message.timestamp)
                {
                    let analysed_message =
                        analysis::analyze_message(parsed_message.message, &emote_scores);

                    finished_messages.push(TwitchChatMessage {
                        channel: parsed_message.channel,
                        sender: parsed_message.sender,
                        message: analysed_message.message,
                        message_score: analysed_message.message_score,
                        timestamp: parsed_message.timestamp,
                    })
                };

                receipt_handles.push(message_handle);
            }
        }

        if !receipt_handles.is_empty() {
            for handle in receipt_handles {
                queue.acknowledge_message(&handle).await;
            }
        }

        if finished_messages.is_empty() {
            break;
        }

        save_message_batch(&database, finished_messages).await.ok();
    }

    Ok(())
}
