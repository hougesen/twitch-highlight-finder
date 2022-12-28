struct Channels {
    #[allow(dead_code)]
    channel_name: String,
    #[allow(dead_code)]
    channel_id: Option<String>,
}

pub async fn fetch_channels(
    db_client: &mongodb::Database,
) -> Result<Vec<String>, mongodb::error::Error> {
    let collection = db_client.collection::<Channels>("channels");

    let mut channel_id_queue = Vec::new();

    if let Ok(channel_id_queue_bson) = collection.distinct("channel_id", None, None).await {
        if !channel_id_queue_bson.is_empty() {
            for channel in &channel_id_queue_bson {
                if let Some(channel_id) = channel.as_str() {
                    channel_id_queue.push(channel_id.to_string());
                }
            }
        }
    }

    Ok(channel_id_queue)
}
