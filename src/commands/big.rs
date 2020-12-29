use crate::utils::unicode_to_safe_ascii;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::{model::channel::Message, prelude::Context};

#[command]
#[description = r#"Write the following text in emoji"#]
pub async fn big(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let message = crate::utils::find_relative_content(ctx, msg, args).await?;

    let new = message
        .chars()
        .map(|c| format!("{} ", char_to_emoji(c).unwrap_or(c.to_string())))
        .collect::<String>();

    msg.channel_id.say(&ctx, new).await?;

    Ok(())
}

pub fn char_to_emoji(c: char) -> Option<String> {
    match unicode_to_safe_ascii(c)? {
        c @ 'a'..='z' => Some(format!(":regional_indicator_{}:", c)),
        '0' => Some(":zero:".to_string()),
        '1' => Some(":one:".to_string()),
        '2' => Some(":two:".to_string()),
        '3' => Some(":three:".to_string()),
        '4' => Some(":four:".to_string()),
        '5' => Some(":five:".to_string()),
        '6' => Some(":six:".to_string()),
        '7' => Some(":seven:".to_string()),
        '8' => Some(":eight:".to_string()),
        '9' => Some(":nine:".to_string()),
        ' ' => Some(":black_small_square:".to_string()),
        '!' => Some(":exclamation:".to_string()),
        '?' => Some(":question:".to_string()),
        '*' => Some(":asterisk:".to_string()),
        '#' => Some(":hash:".to_string()),
        _ => None,
    }
}
