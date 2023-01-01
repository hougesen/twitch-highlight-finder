#[inline]
pub fn get_platform_url(vod_id: &str) -> String {
    format!("https://twitch.tv/videos/{vod_id}")
}

pub async fn get_download_url(video_url: &str) -> Option<String> {
    let command_result = tokio::process::Command::new("yt-dlp")
        .arg("-g")
        .arg(video_url)
        .stdout(std::process::Stdio::piped())
        .output()
        .await;

    if let Ok(output) = command_result {
        if !output.stdout.is_empty() {
            return String::from_utf8(output.stdout).ok();
        }
    }

    None
}
