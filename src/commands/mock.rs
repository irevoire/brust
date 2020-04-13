use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::{model::channel::Message, prelude::Context};
use std::env;

fn get_image_url(text: &str) -> Result<String, Box<dyn std::error::Error>> {
    let logs = env::var("IMGFLIP")?;
    let logs: Vec<&str> = logs.splitn(2, ':').collect();
    let username = logs[0];
    let password = logs[1];

    let url = format!(
        "username={}&password={}&template_id=102156234&boxes[0][text]=&boxes[1][text]={}",
        username, password, text
    );
    let resp = ureq::post("https://api.imgflip.com/caption_image")
        .set("Content-Type", "application/x-www-form-urlencoded")
        .send_string(&url);
    let url = &resp.into_json()?["data"]["url"];
    if let Some(url) = url.as_str() {
        Ok(url.to_string())
    } else {
        Err("Could not as str".into())
    }
}

#[command]
pub fn mock(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
    let mut new = String::new();
    let mut last = false;
    for c in msg.content.chars().skip("!mock".chars().count()) {
        if !c.is_alphabetic() {
            new.push(c);
            continue;
        }
        if last {
            new.push_str(&c.to_lowercase().to_string());
        } else {
            new.push_str(&c.to_uppercase().to_string());
        }
        last = !last;
    }

    if let Ok(url) = get_image_url(&new) {
        let _ = msg
            .channel_id
            .send_files(&ctx, vec![url.as_str()], |m| m.content(&msg.author));
    } else {
        let _ = msg.reply(&ctx, new);
    }
    let _ = msg.delete(&ctx);
    Ok(())
}
