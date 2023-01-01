use std::collections::{BTreeSet, HashMap};

use mongodb::bson::DateTime;

use crate::db::{
    clips::Clip,
    twitch_messages::{get_vod_message_scores, VodMessageScore},
    twitch_vods::TwitchVodModel,
};

pub async fn analyze_vod(
    db_client: &mongodb::Database,
    vod: TwitchVodModel,
) -> Result<Vec<Clip>, mongodb::error::Error> {
    let scores =
        get_vod_message_scores(db_client, &vod.channel_name, vod.streamed_at, vod.ended_at).await?;

    if scores.is_empty() {
        return Ok(Vec::new());
    }

    let bucket = create_time_buckets(scores);

    let average = calculate_time_buckets_average(&bucket);

    let outliers = find_outlier_timestamps(&bucket, average);

    Ok(create_clips(outliers, &vod))
}

const TIME_BUCKET_SIZE: i64 = 10_000;

/// TODO: switch to sliding window?
fn create_time_buckets<T: IntoIterator<Item = VodMessageScore>>(
    message_scores: T,
) -> HashMap<i64, f64> {
    let mut time_buckets = HashMap::new();

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

fn find_outlier_timestamps(original_bucket: &HashMap<i64, f64>, average: f64) -> BTreeSet<i64> {
    let mut values = Vec::new();

    for s in original_bucket.values() {
        values.push(*s);
    }

    values.sort_by(|a, b| a.total_cmp(b));

    let top_10_percentage = values[(values.len() / 10) * 9] as i64;
    let average_modifier = (average * 1.5) as i64;
    let base = TIME_BUCKET_SIZE / 1000;

    let min = std::cmp::max(top_10_percentage, std::cmp::max(average_modifier, base)) as f64;

    let mut outliers = BTreeSet::new();

    for (k, v) in original_bucket {
        if v > &min {
            outliers.insert(*k);
        }
    }

    outliers
}

fn calculate_time_buckets_average(time_bucket: &HashMap<i64, f64>) -> f64 {
    let mut total = 0.;

    for s in time_bucket.values() {
        total += s;
    }

    total / time_bucket.len() as f64
}

#[inline]
fn timestamp_to_video_timestamp(timestamp: i64, start_time: &DateTime) -> i64 {
    timestamp - start_time.timestamp_millis()
}

fn create_clips(mut timestamps: BTreeSet<i64>, vod: &TwitchVodModel) -> Vec<Clip> {
    let mut clips = Vec::new();

    while !timestamps.is_empty() {
        if let Some(start_time) = timestamps.pop_first() {
            let mut end_time = start_time;

            while timestamps.contains(&(end_time + TIME_BUCKET_SIZE)) {
                end_time += TIME_BUCKET_SIZE;

                timestamps.remove(&end_time);
            }

            timestamps.remove(&start_time);

            clips.push(Clip {
                start_time: timestamp_to_video_timestamp(start_time, &vod.streamed_at),
                end_time: timestamp_to_video_timestamp(end_time, &vod.streamed_at),
                user_id: vod.user_id.clone(),
                vod_id: vod.vod_id.clone(),
            });
        }
    }

    clips
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::db::twitch_messages::VodMessageScore;

    use super::{calculate_time_buckets_average, create_time_buckets};

    #[test]
    fn test_create_time_buckets() {
        assert_eq!(
            HashMap::from([(0, 1.0), (5000, 2.0), (10000, 1.0)]),
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
            HashMap::from([(5000, 3.2), (10000, 2.1)]),
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

    #[test]
    fn test_calculate_time_buckets_average_score() {
        assert_eq!(
            4.0 / 3.0,
            calculate_time_buckets_average(&HashMap::from([(0, 1.0), (5000, 2.0), (10000, 1.0)]))
        );

        assert_eq!(
            (123.12 + 21.121 + 512.12) / 3.0,
            calculate_time_buckets_average(&HashMap::from([
                (0, 123.12),
                (5000, 21.121),
                (10000, 512.12)
            ]))
        );
    }

    #[test]
    fn create_clips() {}
}
