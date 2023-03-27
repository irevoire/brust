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

    crate::repeat_message!(ctx, msg, {
        // we want to free the lock as soon as possible
        let mut rng = data.get::<crate::Random>().unwrap().lock().await;

        let resp: serde_json::Value =
            reqwest::get("https://www.nosdeputes.fr/deputes/enmandat/json")
                .await?
                .json()
                .await?;

        if let serde_json::Value::Array(deputes) = &resp["deputes"] {
            let index = rng.gen_range(0..deputes.len());
            format!(
                "https://www.nosdeputes.fr/depute/photo/{}/60",
                deputes
                    .get(index)
                    .ok_or(anyhow::anyhow!("Can't retrieve deputes at index"))?["depute"]["slug"]
            )
        } else {
            Err(anyhow::anyhow!("Can't extract deputes from json"))?
        }
    });

    Ok(())
}
