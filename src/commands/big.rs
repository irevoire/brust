use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::{model::channel::Message, prelude::Context};

#[command]
pub fn big(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let message = crate::utils::find_relative_content(ctx, msg, args)?;
    let mut new = String::new();
    for c in message.chars() {
        new.push_str(&convert_char_to_emoji(c));
    }
    msg.channel_id.say(&ctx, new)?;
    Ok(())
}

fn convert_char_to_emoji(c: char) -> String {
    match c {
        'a'..='z' | 'A'..='Z' => format!(":regional_indicator_{}:", c.to_ascii_lowercase()),
        'á' | 'à' | 'â' | 'ä' | 'Á' | 'À' | 'Â' | 'Ä' => {
            ":regional_indicator_a:".to_string()
        }
        'é' | 'è' | 'ê' | 'ë' | 'É' | 'È' | 'Ê' | 'Ë' => {
            ":regional_indicator_e:".to_string()
        }
        'í' | 'ì' | 'î' | 'ï' | 'Í' | 'Ì' | 'Î' | 'Ï' => {
            ":regional_indicator_i:".to_string()
        }
        'ó' | 'ò' | 'ô' | 'ö' | 'Ó' | 'Ò' | 'Ô' | 'Ö' => {
            ":regional_indicator_o:".to_string()
        }
        'ú' | 'ù' | 'û' | 'ü' | 'Ú' | 'Ù' | 'Û' | 'Ü' => {
            ":regional_indicator_u:".to_string()
        }
        '0' => ":zero:".to_string(),
        '1' => ":one:".to_string(),
        '2' => ":two:".to_string(),
        '3' => ":three:".to_string(),
        '4' => ":four:".to_string(),
        '5' => ":five:".to_string(),
        '6' => ":six:".to_string(),
        '7' => ":seven:".to_string(),
        '8' => ":eight:".to_string(),
        '9' => ":nine:".to_string(),
        ' ' => ":black_small_square:".to_string(),
        '!' => ":exclamation:".to_string(),
        '?' => ":question:".to_string(),
        '*' => ":asterisk:".to_string(),
        '#' => ":hash:".to_string(),
        unknown => unknown.to_string(),
    }
}
