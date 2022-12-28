use futures::StreamExt;
use mongodb::{
    bson::{doc, DateTime},
    options::AggregateOptions,
};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TwitchChatMessage {
    pub channel: String,
    pub sender: String,
    pub message: String,
    pub timestamp: mongodb::bson::DateTime,
    pub message_score: f64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct VodMessageScore {
    #[serde(default, rename = "_id")]
    timestamp: String,
    count: u64,
    total_message_score: f64,
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
    ];

    let mut cursor = db_client
        .collection::<TwitchChatMessage>("twitch_messages")
        .aggregate(
            pipeline,
            AggregateOptions::builder().allow_disk_use(true).build(),
        )
        .await?;

    let mut messages: Vec<VodMessageScore> = Vec::new();

    while let Some(next_cursor) = cursor.next().await {
        if let Ok(result) = next_cursor {
            if let Ok(document) = mongodb::bson::from_document::<VodMessageScore>(result) {
                messages.push(document)
            }
        }
    }

    Ok(messages)
}
