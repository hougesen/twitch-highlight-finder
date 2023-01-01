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

pub async fn download_video(
    download_url: &str,
    file_name: &str,
    start: i64,
    duration: i64,
) -> Result<std::process::Output, std::io::Error> {
    tokio::process::Command::new("ffmpeg")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .args(["-ss", &start.to_string()])
        .args(["-t", &duration.to_string()])
        .arg("-y")
        .args(["-i", download_url, &format!("5second-{file_name}.mp4")])
        .kill_on_drop(true)
        .output()
        .await
}
