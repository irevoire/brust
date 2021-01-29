use serenity::framework::standard::{Args, CommandError};
use serenity::model::prelude::UserId;
use serenity::{model::channel::Message, prelude::Context};

/// You can execute what you want in the block, but it should return a `serenity::model::channel::Message`.
/// Then an emoji will be sent under the message, and while someone click on this emoji the block
/// will be repeated.
/// It also needs to get a `serenity::prelude::Context`.
/// Here is an example with the command fox:
/// ```no_run
/// #[command]
/// pub async fn fox(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
///     crate::repeat_message!(ctx, {
///         let url = fetch_random_fox_url().await?;
///
///         msg.channel_id
///             .send_files(&ctx, vec![url.as_str()], |m| m.content(&msg.author))
///             .await?
///     });
///
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! repeat_message {
    ($ctx:ident, $msg:ident, $code:block) => {
        let mut author = $msg.author.clone();

        loop {
            use serenity::model::channel::ReactionType;
            let plus_emoji = "➕".parse::<ReactionType>().unwrap();

            let url = $code;
            let answer = $msg
                .channel_id
                .send_files(&$ctx, vec![url.as_str()], |m| m.content(&author))
                .await?;

            answer.react($ctx, plus_emoji.clone()).await?;

            let tmp_plus_emoji = plus_emoji.clone();

            let more = answer
                .await_reaction($ctx)
                .timeout(std::time::Duration::from_secs(60 * 10))
                .filter(move |reaction| reaction.emoji == tmp_plus_emoji)
                .await;

            answer.delete_reaction_emoji($ctx, plus_emoji).await?;
            if more.is_none() {
                break;
            } else {
                author = more
                    .unwrap()
                    .as_inner_ref()
                    .user_id
                    .unwrap()
                    .to_user($ctx)
                    .await?
                    .clone();
            }
        }
    };
}
pub async fn find_relative_content(
    ctx: &Context,
    msg: &Message,
    args: Args,
) -> Result<String, CommandError> {
    let mut message = None;
    if args.is_empty() {
        message = Some(
            msg.channel_id
                .messages(&ctx, |retriever| retriever.before(msg.id).limit(1))
                .await?[0]
                .content
                .clone(),
        );
    } else if let Ok(userid) = args.current().unwrap().parse::<UserId>() {
        message = msg
            .channel_id
            .messages(&ctx, |retriever| retriever.before(msg.id))
            .await?
            .iter()
            .find(|msg| msg.author.id == userid)
            .map(|msg| msg.content.clone())
    }
    let message = message.unwrap_or_else(|| args.rest().to_string());
    Ok(message)
}

pub fn unicode_to_safe_ascii(c: char) -> Option<char> {
    match c {
        'a'..='z' | 'A'..='Z' | '0'..='9' | ' ' | '.' | '!' | '?' | '*' | '#' | '\'' => {
            Some(c.to_ascii_lowercase())
        }
        'á' | 'à' | 'â' | 'ä' | 'Á' | 'À' | 'Â' | 'Ä' => Some('a'),
        'é' | 'è' | 'ê' | 'ë' | 'É' | 'È' | 'Ê' | 'Ë' => Some('e'),
        'í' | 'ì' | 'î' | 'ï' | 'Í' | 'Ì' | 'Î' | 'Ï' => Some('i'),
        'ó' | 'ò' | 'ô' | 'ö' | 'Ó' | 'Ò' | 'Ô' | 'Ö' => Some('o'),
        'ú' | 'ù' | 'û' | 'ü' | 'Ú' | 'Ù' | 'Û' | 'Ü' => Some('u'),
        'ç' | 'Ç' => Some('c'),
        '«' | '»' | '“' | '”' | '„' => Some('"'),
        '’' => Some('\''),
        _ => None,
    }
}
