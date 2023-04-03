use rand::Rng;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context,
};

#[command]
#[description = "Random image of French deputee"]
pub async fn an(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let data = ctx.data.read().await;

    // crate::repeat_message!(ctx, msg, {
    // we want to free the lock as soon as possible
    let mut rng = data.get::<crate::Random>().unwrap().lock().await;

    let resp: serde_json::Value = reqwest::get("https://www.nosdeputes.fr/deputes/enmandat/json")
        .await?
        .json()
        .await?;

    if let serde_json::Value::Array(deputes) = &resp["deputes"] {
        let index = rng.gen_range(0..deputes.len());
        let slug = deputes
            .get(index)
            .ok_or_else(|| anyhow::anyhow!("Can't retrieve deputes at index {index}"))?["depute"]
            ["slug"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("No slug for depute index {index}"))?;
        let url = format!("https://www.nosdeputes.fr/depute/photo/{slug}/250");

        let bytes = reqwest::get(url).await?.bytes().await?;

        msg.channel_id
            .send_files(
                &ctx,
                vec![(bytes.as_ref(), format!("{slug}.png").as_str())],
                |m| m.content(&msg.author),
            )
            .await?;
    } else {
        Err(anyhow::anyhow!("Can't extract deputes from json"))?;
    }
    // });

    Ok(())
}
