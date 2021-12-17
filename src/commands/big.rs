use crate::utils::unicode_to_safe_ascii;
use anyhow::{anyhow, Result};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::interactions::application_command::ApplicationCommandInteraction;
use serenity::model::interactions::InteractionResponseType;
use serenity::{model::channel::Message, prelude::Context};

#[command]
#[usage("[your text]")]
#[example("fat brain")]
#[description = r#"Write the following text in emoji"#]
pub async fn big(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let message = crate::utils::find_relative_content(ctx, msg, args).await?;

    msg.channel_id.say(&ctx, biggify(&message)).await?;

    Ok(())
}

impl crate::Handler {
    pub async fn handle_big(
        &self,
        ctx: Context,
        command: ApplicationCommandInteraction,
    ) -> Result<()> {
        let option = &command.data.options[0];

        let value = option
            .value
            .as_ref()
            .ok_or(anyhow!("text is a required argument"))?
            .as_str()
            .ok_or(anyhow!("text is always a String type"))?;

        let uwud = biggify(value);

        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|data| data.content(uwud))
            })
            .await?;

        Ok(())
    }
}

fn biggify(text: &str) -> String {
    text.chars()
        .map(|c| format!("{} ", char_to_emoji(c).unwrap_or(c.to_string())))
        .collect()
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
