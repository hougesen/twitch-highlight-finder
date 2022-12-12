use futures::{future, StreamExt};
use mongodb::{bson::oid::ObjectId, change_stream::event::OperationType};

mod analyzer;
mod db;

#[tokio::main]
async fn main() -> Result<(), mongodb::error::Error> {
    let db_client = db::get_db_client().await.unwrap();

    let emote_scores = db::get_emote_scores(&db_client).await.into_read_only();

    if emote_scores.is_empty() {
        panic!("Emote score is empty!")
    }

    println!("analyze_pending");

    analyze_pending(db_client.clone(), emote_scores.clone())
        .await
        .expect("Something went wrong analyzing past messages");

    // println!("watch_messages");

    // NOTE: disabled since this service is run as a cronjob
    // watch_messages(db_client, emote_scores)
    //    .await
    //    .expect("Something went wrong watching messages");

    Ok(())
}

async fn handle_new_message(
    db_client: &mongodb::Database,
    emote_scores: &dashmap::ReadOnlyView<String, u8>,
    message_id: ObjectId,
    message: String,
) -> Result<mongodb::results::UpdateResult, mongodb::error::Error> {
    let analyzed_message = analyzer::analyze_message(message, emote_scores);

    db::save_message_score(
        db_client.clone(),
        message_id,
        analyzed_message.message_score,
    )
    .await
}

async fn analyze_pending(
    db_client: mongodb::Database,
    emote_scores: dashmap::ReadOnlyView<String, u8>,
) -> Result<(), mongodb::error::Error> {
    let messages = db::get_pending_chat_messages(&db_client).await;

    println!("message len: {}", messages.len());
    let mut raw_futs = vec![];

    for m in messages {
        raw_futs.push(handle_new_message(
            &db_client,
            &emote_scores,
            m.id,
            m.message,
        ));
    }

    let unpin_futs: Vec<_> = raw_futs.into_iter().map(Box::pin).collect();

    let mut futs = unpin_futs;

    while !futs.is_empty() {
        match future::select_all(futs).await {
            (Ok(_), _index, remaining) => {
                if remaining.len() % 100 == 0 {
                    println!("remaining: {}", remaining.len());
                }

                futs = remaining;
            }
            (Err(_e), _index, remaining) => {
                if remaining.len() % 100 == 0 {
                    println!("remaining: {}", remaining.len());
                }

                futs = remaining;
            }
        }
    }

    Ok(())
}

async fn watch_messages(
    db_client: mongodb::Database,
    emote_scores: dashmap::ReadOnlyView<String, u8>,
) -> Result<(), mongodb::error::Error> {
    let collection = db_client.collection::<db::TwitchChatMessage>("twitch_messages");

    let pipeline = vec![mongodb::bson::doc! {
        "$match": {
            "message_score": {
                "$exists": false
            }
        }
    }];

    let mut change_stream = collection.watch(pipeline, None).await?;

    while let Some(event) = change_stream.next().await.transpose()? {
        println!("operation performed: {:?}", event.operation_type);

        if event.operation_type == OperationType::Insert {
            if let Some(document) = event.full_document {
                handle_new_message(&db_client, &emote_scores, document.id, document.message)
                    .await
                    .ok();
            }
        }
    }

    Ok(())
}
