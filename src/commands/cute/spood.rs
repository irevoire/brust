use anyhow::{anyhow, Result};
use rand::prelude::*;
use select::document::Document;
use select::predicate::Attr;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::{
    model::channel::{Message, ReactionType},
    prelude::Context,
};
use std::time::Duration;

#[command]
#[aliases("spoddo")]
#[usage("")]
#[example("")]
#[description = "Send cute spood pictures stolen from https://spiderid.com"]
pub async fn spood(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let data = ctx.data.read().await;

    loop {
        let url = {
            // we want to free the lock as soon as possible
            let mut rng = data.get::<crate::Random>().unwrap().lock().await;

            let page = fetch_spood_page(&mut *rng).await.map_err(|e| {
                anyhow!(
                    "Spoddo express: was not able to deliver you spood: {}\n{}",
                    e,
                    "https://cdn.drawception.com/drawings/gB8gGBpkSW.png" // crying spoddo
                )
            })?;

            fetch_url_in_spood_page(page, &mut *rng).ok_or(anyhow!(
                "Spoddo express: your spood got lost in the page :pensive:"
            ))?
        };

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

async fn fetch_spood_page(rng: &mut impl Rng) -> Result<String> {
    let nb_pictures = 64888;
    let pictures_per_pages = 50;
    let page_number = rng.gen_range(0..nb_pictures / pictures_per_pages);

    Ok(reqwest::get(&format!(
        "https://spiderid.com/pictures/?fwp_paged={}",
        page_number
    ))
    .await?
    .text()
    .await?)
}

fn fetch_url_in_spood_page(page: String, rng: &mut impl Rng) -> Option<String> {
    let document = Document::from(page.as_str());
    Some(
        document
            .find(Attr("class", "picCardThumb"))
            .choose(rng)?
            .attr("src")?
            .into(),
    )
}
