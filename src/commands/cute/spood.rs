use anyhow::{anyhow, Result};
use rand::prelude::*;
use select::document::Document;
use select::predicate::Attr;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::{model::channel::Message, prelude::Context};

#[command]
#[aliases("spoddo")]
#[description = "Send cute spood pictures stolen from https://spiderid.com"]
pub async fn spood(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let mut rng = data.get::<crate::Random>().unwrap().lock().await;

    let page = fetch_spood_page(&mut *rng).await.map_err(|e| {
        anyhow!(
            "Spoddo express: was not able to deliver you spood: {}\n{}",
            e,
            "https://cdn.drawception.com/drawings/gB8gGBpkSW.png" // crying spoddo
        )
    })?;

    let url = fetch_url_in_spood_page(page, &mut *rng).ok_or(anyhow!(
        "Spoddo express: your spood got lost in the page :pensive:"
    ))?;

    msg.channel_id
        .send_files(&ctx, vec![url.as_str()], |m| m.content(&msg.author))
        .await?;
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
