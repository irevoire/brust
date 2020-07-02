use reqwest::blocking::Client;
use select::document::Document;
use select::predicate::Attr;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::{model::channel::Message, prelude::Context};

#[command]
#[description = "Send cute dog picture stolen from https://random.dog"]
pub fn dog(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
    let page = fetch_dog_page();
    if let Err(e) = page {
        let _ = msg.reply(
            &ctx,
            format!(
                "Doggo express: your doggo was not able to come because: {}",
                e
            ),
        );
        return Ok(());
    }
    let url = fetch_url_in_dog_page(page?);
    if url.is_none() {
        let _ = msg.reply(
            &ctx,
            format!("Doggo express: your doggo got lost :pensive:"),
        );
        return Ok(());
    }
    let url = url.unwrap();
    let _ = msg
        .channel_id
        .send_files(&ctx, vec![url.as_str()], |m| m.content(&msg.author));
    Ok(())
}

fn fetch_dog_page() -> Result<String, Box<dyn std::error::Error>> {
    Ok(Client::new().get("https://random.dog").send()?.text()?)
}

fn fetch_url_in_dog_page(page: String) -> Option<String> {
    let document = Document::from(page.as_str());
    let dog_img = document.find(Attr("id", "dog-img")).next()?;
    let url = dog_img.find(Attr("src", ())).next()?.attr("src")?;
    Some(format!("https://random.dog/{}", url))
}
