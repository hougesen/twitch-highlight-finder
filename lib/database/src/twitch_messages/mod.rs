use futures::StreamExt;
use mongodb::{
    bson::{doc, DateTime},
    options::{AggregateOptions, IndexOptions, InsertManyOptions},
    results::{CreateIndexResult, InsertManyResult},
};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TwitchChatMessage {
    pub channel: String,
    pub sender: String,
    pub message: String,
    pub timestamp: mongodb::bson::DateTime,
    pub message_score: f64,
}

pub async fn save_messages(
    db_client: &mongodb::Database,
    messages: impl IntoIterator<Item = TwitchChatMessage>,
) -> Result<InsertManyResult, mongodb::error::Error> {
    let collection = db_client.collection::<TwitchChatMessage>("twitch_messages");

    collection
        .insert_many(
            messages,
            InsertManyOptions::builder().ordered(Some(false)).build(),
        )
        .await
}

/// Have to make a unique compound index since SQS standard queues can have duplications
pub async fn ensure_message_index_exists(
    db_client: &mongodb::Database,
) -> Result<CreateIndexResult, mongodb::error::Error> {
    let collection = db_client.collection::<TwitchChatMessage>("twitch_messages");

    collection
        .create_index(
            mongodb::IndexModel::builder()
                .keys(doc! { "channel": 1, "sender": 1, "message": 1, "timestamp": 1 })
                .options(IndexOptions::builder().unique(true).build())
                .build(),
            None,
        )
        .await
}

#[derive(serde::Deserialize)]
pub struct VodMessageScore {
    pub timestamp: DateTime,
    pub count: u64,
    pub total_message_score: f64,
}

pub async fn get_vod_message_scores(
    db_client: &mongodb::Database,
    channel_name: &str,
    started_at: DateTime,
    ended_at: DateTime,
) -> Result<Vec<VodMessageScore>, mongodb::error::Error> {
    let pipeline = [
        doc! {
            "$match": {
                "channel": channel_name,
                "timestamp": {
                    "$gte": started_at,
                    "$lte": ended_at
                }
            }
        },
        doc! {
            "$group": {
                "_id": {
                    "$dateToString": {
                        "format": "%Y-%m-%dT%H:%M:%S",
                        "date": "$timestamp"
                    }
                },
                "count": { "$sum": 1 },
                "total_message_score": { "$sum": "$message_score" }
            }
        },
        doc! {
            "$addFields": {
                "timestamp": {
                    "$dateFromString": {
                        "dateString": "$_id"
                    }
                }
            }
        },
    ];

    let mut cursor = db_client
        .collection::<TwitchChatMessage>("twitch_messages")
        .aggregate(
            pipeline,
            AggregateOptions::builder().allow_disk_use(true).build(),
        )
        .await?;

    let mut messages = Vec::new();

    while let Some(next_cursor) = cursor.next().await {
        if let Ok(result) = next_cursor {
            if let Ok(document) = mongodb::bson::from_document::<VodMessageScore>(result) {
                messages.push(document)
            }
        }
    }

    Ok(messages)
}
