use rand::Rng;
use reqwest::blocking::Client;
use select::document::Document;
use select::predicate::Attr;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::{model::channel::Message, prelude::Context};

#[command]
#[description = "Send cute cat picture stolen from http://random.cat"]
pub fn cat(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
    let data = ctx.data.read();
    let mut rng = data.get::<crate::Random>().unwrap().lock().unwrap();

    let page = fetch_cat_page(&mut *rng);
    if let Err(e) = page {
        let _ = msg.reply(
            &ctx,
            format!(
                "Catto express: was not able to deliver you cat: {}\n{}",
                e,
                "https://i.redd.it/4q32jedhkgi31.jpg" // crying catto
            ),
        );
        return Ok(());
    }

    let url = fetch_url_in_cat_page(page?);
    if url.is_none() {
        let _ = msg.reply(
            &ctx,
            format!("Catto express: your catto got lost in the page :pensive:"),
        );
        return Ok(());
    }
    let url = url.unwrap();

    let _ = msg
        .channel_id
        .send_files(&ctx, vec![url.as_str()], |m| m.content(&msg.author));
    Ok(())
}

fn fetch_cat_page(rng: &mut impl Rng) -> Result<String, Box<dyn std::error::Error>> {
    let cat_id = rng.gen_range(0, 1677);
    Ok(Client::new()
        .get(&format!("http://random.cat/view/{}", cat_id))
        .send()?
        .text()?)
}

fn fetch_url_in_cat_page(page: String) -> Option<String> {
    let document = Document::from(page.as_str());
    Some(document.find(Attr("id", "cat")).next()?.attr("src")?.into())
}
