use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::{
    model::channel::{Message, ReactionType},
    prelude::Context,
};

#[command]
#[description = r#"Create a poll under the previous message.
If no options are provided it'll use the following options: ✅ or ❎.
You can also provide your own emoji for the poll, anything that can't be converted to an emoji will be ignored"#]
#[usage("[List of things you want people to choose from]*")]
#[example("")]
#[example("😇 😈")]
#[example("🐺 🦊 🦁 🐯")]
pub async fn poll(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let message = msg
        .channel_id
        .messages(&ctx, |retriever| retriever.before(msg.id).limit(1))
        .await?[0]
        .clone();

    let mut reacted = false;

    for emoji in args.iter::<ReactionType>().filter_map(|a| a.ok()) {
        message.react(&ctx, emoji).await?;
        reacted = true;
    }

    if reacted == false {
        message
            .react(&ctx, "✅".parse::<ReactionType>().unwrap())
            .await?;
        message
            .react(&ctx, "❎".parse::<ReactionType>().unwrap())
            .await?;
    }

    msg.delete(&ctx).await?;

    Ok(())
}
