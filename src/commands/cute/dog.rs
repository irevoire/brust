use reqwest::blocking::Client;
use select::document::Document;
use select::predicate::Attr;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::{model::channel::Message, prelude::Context};

#[command]
#[aliases("doggo")]
#[description = "Send cute dog picture stolen from https://random.dog"]
pub fn dog(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let url = if args.len() != 0 {
        fetch_dog_breed_url(args.raw().collect::<Vec<&str>>())
    } else {
        fetch_random_dog_url()
    };
    match url {
        Ok(url) => msg
            .channel_id
            .send_files(&ctx, vec![url.as_str()], |m| m.content(&msg.author)),
        Err(annoncement) => msg.reply(&ctx, format!("Doggo express: {}", annoncement)),
    };
    Ok(())
}

/// look for a specific breed of dog from dog api
fn fetch_dog_breed_url(mut breed: Vec<&str>) -> Result<String, Box<dyn std::error::Error>> {
    breed.reverse();
    let breed = breed.join("/");
    let resp: serde_json::Value = Client::new()
        .get(&format!(
            "https://dog.ceo/api/breed/{}/images/random",
            breed
        ))
        .send()?
        .json()?;

    match resp["status"].as_str().unwrap() {
        "error" => Err(resp["message"].as_str().unwrap().into()),
        "success" => Ok(resp["message"].as_str().unwrap().to_string()),
        _ => Err("The doggo center looks closed".into()),
    }
}

/// return an url from http://random.dog
fn fetch_random_dog_url() -> Result<String, Box<dyn std::error::Error>> {
    let page = fetch_dog_page()?;
    let url = fetch_url_in_dog_page(page).ok_or("your doggo got lost :pensive:")?;
    Ok(url)
}

fn fetch_dog_page() -> Result<String, Box<dyn std::error::Error>> {
    Ok(Client::new().get("https://random.dog").send()?.text()?)
}

fn fetch_url_in_dog_page(page: String) -> Option<String> {
    let document = Document::from(page.as_str());
    let dog_img = document.find(Attr("id", "dog-img")).next()?;
    let url = dog_img
        .attr("src")
        .or_else(|| dog_img.find(Attr("src", ())).next()?.attr("src"))?;
    Some(format!("https://random.dog/{}", url))
}
