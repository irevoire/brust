use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::{model::channel::Message, prelude::Context};

#[command]
#[aliases("roast")]
#[description = r#"Roasted kid meme.
You can:
    - Type your message right after the `!roasted`
    - @someone to use his message
    - Write nothing to use the last message in the channel"#]
pub fn roasted(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let message = crate::utils::find_relative_content(ctx, msg, args)?;
    if let Ok(url) = crate::imgflip::generate_image_url(None, Some(&message), "122616222") {
        let _ = msg
            .channel_id
            .send_files(&ctx, vec![url.as_str()], |m| m.content(&msg.author));
    }
    let _ = msg.delete(&ctx);
    Ok(())
}
