use anyhow::{bail, Result};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::{
    model::channel::{Message, ReactionType},
    prelude::Context,
};
use std::time::Duration;

#[command]
#[aliases("foxxo")]
#[description = "Send cute fox picture stolen from https://randomfox.ca"]
#[usage("")]
#[example("")]
pub async fn fox(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    loop {
        let url = fetch_random_fox_url().await?;

        let answer = msg
            .channel_id
            .send_files(&ctx, vec![url.as_str()], |m| m.content(&msg.author))
            .await?;

        let plus_emoji = "➕".parse::<ReactionType>().unwrap();

        answer.react(ctx, plus_emoji.clone()).await?;

        let more = answer
            .await_reaction(ctx)
            .timeout(Duration::from_secs(60 * 10))
            .filter(move |reaction| reaction.emoji == plus_emoji)
            .await;

        if more.is_none() {
            break;
        }
    }

    Ok(())
}

/// return an url from http://randomfox.ca
async fn fetch_random_fox_url() -> Result<String> {
    let foxy_thingy: serde_json::Value = reqwest::Client::new()
        .get("https://randomfox.ca/floof/")
        .send()
        .await?
        .json()
        .await?;
    let url = &foxy_thingy["image"];
    if let Some(url) = url.as_str() {
        Ok(url.to_string())
    } else {
        bail!("Could not as str")
    }
}
