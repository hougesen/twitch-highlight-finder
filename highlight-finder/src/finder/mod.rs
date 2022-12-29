use crate::db::{
    twitch_messages::{get_vod_message_scores, VodMessageScore},
    twitch_vods::{mark_as_analyzed, TwitchVodModel},
};

pub async fn analyze_vod(
    db_client: &mongodb::Database,
    vod: TwitchVodModel,
) -> Result<(), mongodb::error::Error> {
    let scores =
        get_vod_message_scores(db_client, &vod.channel_name, vod.streamed_at, vod.ended_at).await?;

    println!("{} scores: {:#?}", vod.id, scores.len());

    if scores.is_empty() {
        mark_as_analyzed(db_client, vod.id).await?;
        return Ok(());
    }

    let time_buckets = create_time_buckets(scores);

    Ok(())
}

const TIME_BUCKET_SIZE: i64 = 5000;

type TimeBucket = std::collections::HashMap<i64, f64>;

pub fn create_time_buckets<T: IntoIterator<Item = VodMessageScore>>(
    message_scores: T,
) -> TimeBucket {
    let mut time_buckets = std::collections::HashMap::new();

    for score in message_scores {
        let timestamp_ms = score.timestamp.timestamp_millis();
        let key = timestamp_ms - (timestamp_ms % TIME_BUCKET_SIZE);

        time_buckets
            .entry(key)
            .and_modify(|v| *v += score.total_message_score)
            .or_insert(score.total_message_score);
    }

    time_buckets
}

#[cfg(test)]
mod tests {
    use crate::db::twitch_messages::VodMessageScore;

    use super::create_time_buckets;

    #[test]
    fn test_create_time_buckets() {
        assert_eq!(
            std::collections::HashMap::from([(0, 1.0), (5000, 2.0), (10000, 1.0)]),
            create_time_buckets([
                VodMessageScore {
                    timestamp: mongodb::bson::DateTime::from_millis(0),
                    count: 1,
                    total_message_score: 1.0,
                },
                VodMessageScore {
                    timestamp: mongodb::bson::DateTime::from_millis(5000),
                    count: 1,
                    total_message_score: 1.0,
                },
                VodMessageScore {
                    timestamp: mongodb::bson::DateTime::from_millis(5001),
                    count: 1,
                    total_message_score: 1.0,
                },
                VodMessageScore {
                    timestamp: mongodb::bson::DateTime::from_millis(10001),
                    count: 1,
                    total_message_score: 1.0,
                },
            ])
        );

        // test that order doesn't matter
        assert_eq!(
            std::collections::HashMap::from([(5000, 3.2), (10000, 2.1)]),
            create_time_buckets([
                VodMessageScore {
                    timestamp: mongodb::bson::DateTime::from_millis(10001),
                    count: 1,
                    total_message_score: 1.0,
                },
                VodMessageScore {
                    timestamp: mongodb::bson::DateTime::from_millis(10000),
                    count: 1,
                    total_message_score: 1.1,
                },
                VodMessageScore {
                    timestamp: mongodb::bson::DateTime::from_millis(5000),
                    count: 1,
                    total_message_score: 1.0,
                },
                VodMessageScore {
                    timestamp: mongodb::bson::DateTime::from_millis(5001),
                    count: 1,
                    total_message_score: 1.0,
                },
                VodMessageScore {
                    timestamp: mongodb::bson::DateTime::from_millis(5001),
                    count: 1,
                    total_message_score: 1.2,
                },
            ])
        );
    }
}
