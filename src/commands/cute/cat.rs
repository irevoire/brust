use rand::Rng;
use reqwest::blocking::Client;
use select::document::Document;
use select::predicate::Attr;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::{model::channel::Message, prelude::Context};

#[command]
#[description = "Send cute cat picture stolen from http://random.cat"]
pub fn cat(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
    let mut rng = rand::thread_rng();
    let cat_id = rng.gen_range(0, 1677);
    let page: String = Client::new()
        .get(&format!("http://random.cat/view/{}", cat_id))
        .send()?
        .text()?;
    let document = Document::from(page.as_str());
    let url = document
        .find(Attr("id", "cat"))
        .next()
        .unwrap()
        .attr("src")
        .unwrap();
    let _ = msg
        .channel_id
        .send_files(&ctx, vec![url], |m| m.content(&msg.author));
    Ok(())
}
