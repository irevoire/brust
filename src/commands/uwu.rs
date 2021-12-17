use anyhow::{anyhow, Result};
use serenity::{
    client::Context,
    model::interactions::{
        application_command::ApplicationCommandInteraction, InteractionResponseType,
    },
};

use crate::Handler;

impl Handler {
    pub async fn handle_uwuify(
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

        let uwud = uwuifier::uwuify_str_sse(value);

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
