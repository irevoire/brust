use anyhow::{anyhow, bail, Result};
use select::document::Document;
use select::predicate::Attr;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::{model::channel::Message, prelude::Context};

#[command]
#[aliases("doggo")]
#[description = "Send cute dog picture stolen from https://random.dog"]
#[usage("[sub race] [main race]")]
#[example("")]
#[example("golden retriever")]
#[example("retriever")]
#[example("bernese mountain")]
pub async fn dog(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    crate::repeat_message!(ctx, msg, {
        if args.len() != 0 {
            fetch_dog_breed_url(args.raw().collect::<Vec<&str>>()).await
        } else {
            fetch_random_dog_url().await
        }?
    });

    Ok(())
}

/// look for a specific breed of dog from dog api
async fn fetch_dog_breed_url(mut breed: Vec<&str>) -> Result<String> {
    breed.reverse();
    let breed = breed.join("/");
    let resp: serde_json::Value = reqwest::get(&format!(
        "https://dog.ceo/api/breed/{}/images/random",
        breed
    ))
    .await?
    .json()
    .await?;

    match resp["status"].as_str().unwrap() {
        "error" => bail!("{}", resp["message"]),
        "success" => Ok(resp["message"].as_str().unwrap().to_string()),
        _ => bail!("The doggo center looks closed"),
    }
}

/// return an url from http://random.dog
async fn fetch_random_dog_url() -> Result<String> {
    let page = fetch_dog_page().await?;
    let url = fetch_url_in_dog_page(page).ok_or(anyhow!("your doggo got lost :pensive:"))?;
    Ok(url)
}

async fn fetch_dog_page() -> Result<String> {
    Ok(reqwest::get("https://random.dog").await?.text().await?)
}

fn fetch_url_in_dog_page(page: String) -> Option<String> {
    let document = Document::from(page.as_str());
    let dog_img = document.find(Attr("id", "dog-img")).next()?;
    let url = dog_img
        .attr("src")
        .or_else(|| dog_img.find(Attr("src", ())).next()?.attr("src"))?;
    Some(format!("https://random.dog/{}", url))
}
