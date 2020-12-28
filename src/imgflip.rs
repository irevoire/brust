use anyhow::{bail, Result};
use reqwest::header;
use std::env;

/// Generate a meme with the imageflip API
/// h1 -> the text to the top of the image
/// h2 -> the text to the bottom of the image
/// id -> the id of the meme
pub async fn generate_image_url(h1: Option<&str>, h2: Option<&str>, id: &str) -> Result<String> {
    let logs = env::var("IMGFLIP")?;
    let logs: Vec<&str> = logs.splitn(2, ':').collect();
    let username = logs[0];
    let password = logs[1];

    let url = format!(
        "username={}&password={}&template_id={}&boxes[0][text]={}&boxes[1][text]={}",
        username,
        password,
        id,
        h1.unwrap_or(""),
        h2.unwrap_or("")
    );
    let resp: serde_json::Value = reqwest::Client::new()
        .post("https://api.imgflip.com/caption_image")
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(url)
        .send()
        .await?
        .json()
        .await?;
    let url = &resp["data"]["url"];
    if let Some(url) = url.as_str() {
        Ok(url.to_string())
    } else {
        bail!("Could not as str")
    }
}
