use serenity::framework::standard::{Args, CommandError};
use serenity::model::prelude::UserId;
use serenity::{model::channel::Message, prelude::Context};

pub fn find_relative_content(
    ctx: &mut Context,
    msg: &Message,
    args: Args,
) -> Result<String, CommandError> {
    let mut message = None;
    if args.is_empty() {
        message = Some(
            msg.channel_id
                .messages(&ctx, |retriever| retriever.before(msg.id).limit(1))?[0]
                .content
                .clone(),
        );
    } else if let Ok(userid) = args.current().unwrap().parse::<UserId>() {
        message = msg
            .channel_id
            .messages(&ctx, |retriever| retriever.before(msg.id))?
            .iter()
            .find(|msg| msg.author.id == userid)
            .map(|msg| msg.content.clone())
    }
    let message = message.unwrap_or_else(|| args.rest().to_string());
    Ok(message)
}
