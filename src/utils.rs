use serenity::framework::standard::{Args, CommandError};
use serenity::model::prelude::UserId;
use serenity::{model::channel::Message, prelude::Context};

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
        'a'..='z' | 'A'..='Z' | '0'..='9' | ' ' | '.' | '!' | '?' | '*' | '#' => {
            Some(c.to_ascii_lowercase())
        }
        'á' | 'à' | 'â' | 'ä' | 'Á' | 'À' | 'Â' | 'Ä' => Some('a'),
        'é' | 'è' | 'ê' | 'ë' | 'É' | 'È' | 'Ê' | 'Ë' => Some('e'),
        'í' | 'ì' | 'î' | 'ï' | 'Í' | 'Ì' | 'Î' | 'Ï' => Some('i'),
        'ó' | 'ò' | 'ô' | 'ö' | 'Ó' | 'Ò' | 'Ô' | 'Ö' => Some('o'),
        'ú' | 'ù' | 'û' | 'ü' | 'Ú' | 'Ù' | 'Û' | 'Ü' => Some('u'),
        'ç' | 'Ç' => Some('c'),
        _ => None,
    }
}
