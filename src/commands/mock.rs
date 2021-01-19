use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::{model::channel::Message, prelude::Context};

#[command]
#[usage("[select a message | your text]")]
#[example("")]
#[example("@machin")]
#[example("hello")]
#[description = r#"Mocking sponge bob meme.
You can:
    - Type your message right after the `!mock`
    - @someone to use his message
    - Write nothing to use the last message in the channel"#]
pub async fn mock(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let message = crate::utils::find_relative_content(ctx, msg, args).await?;
    let mut new = String::new();
    let mut last = false;
    for c in message.chars() {
        if !c.is_alphabetic() {
            new.push(c);
            continue;
        }
        if last {
            new.push_str(&c.to_lowercase().to_string());
        } else {
            new.push_str(&c.to_uppercase().to_string());
        }
        last = !last;
    }

    if let Ok(url) = crate::imgflip::generate_image_url(None, Some(&new), "102156234").await {
        let _ = msg
            .channel_id
            .send_files(&ctx, vec![url.as_str()], |m| m.content(&msg.author))
            .await?;
    } else {
        msg.reply(&ctx, new).await?;
    }
    msg.delete(&ctx).await?;
    Ok(())
}
