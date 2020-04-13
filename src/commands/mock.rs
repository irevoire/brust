use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::{model::channel::Message, prelude::Context};

#[command]
pub fn mock(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let message = crate::utils::find_relative_content(ctx, msg, args)?;
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

    if let Ok(url) = crate::imgflip::generate_image_url(None, Some(&new), "102156234") {
        let _ = msg
            .channel_id
            .send_files(&ctx, vec![url.as_str()], |m| m.content(&msg.author));
    } else {
        let _ = msg.reply(&ctx, new);
    }
    let _ = msg.delete(&ctx);
    Ok(())
}
