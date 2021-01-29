use anyhow::{bail, Result};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::{model::channel::Message, prelude::Context};

#[command]
#[aliases("foxxo")]
#[description = "Send cute fox picture stolen from https://randomfox.ca"]
#[usage("")]
#[example("")]
pub async fn fox(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    crate::repeat_message!(ctx, msg, { fetch_random_fox_url().await? });

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
