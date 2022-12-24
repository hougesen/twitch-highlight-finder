mod analysis;
mod db;
mod parser;
mod queue;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_client = db::get_db_client().await.unwrap();

    let emote_scores = db::get_emote_scores(&db_client).await.into_read_only();

    if emote_scores.is_empty() {
        panic!("Emote score is empty!")
    }

    db::ensure_message_index_exists(&db_client).await?;

    let mut queue = queue::Queue::new(None).await;

    let created_queue = queue.create_queue("unparsed-messages").await?;

    queue.set_queue_url(created_queue.queue_url().unwrap());

    while !queue.empty().await {
        let mut finished_messages: Vec<db::TwitchChatMessage> = Vec::new();

        if let Ok(queue_messages) = queue.get_message_batch(Some(10)).await {
            for queue_message in queue_messages {
                if let Some(parsed_message) =
                    parser::parse_message(queue_message.message, queue_message.timestamp)
                {
                    let analysed_message =
                        analysis::analyze_message(parsed_message.message, &emote_scores);

                    finished_messages.push(db::TwitchChatMessage {
                        channel: parsed_message.channel,
                        sender: parsed_message.sender,
                        message: analysed_message.message,
                        message_score: analysed_message.message_score,
                        timestamp: parsed_message.timestamp,
                    })
                }
            }
        }

        if !finished_messages.is_empty() {
            db::save_message_batch(&db_client, finished_messages)
                .await
                .ok();
        }
    }

    println!("queue is empty now");

    Ok(())
}
