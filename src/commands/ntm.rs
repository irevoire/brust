use serenity::framework::standard::{macros::command, Args, CommandResult};

use serenity::{
    model::channel::Message,
    prelude::{Context, TypeMapKey},
};

pub struct Ntm;

// we are going to store the insults in the first vector and random index in the second
impl TypeMapKey for Ntm {
    type Value = Vec<&'static str>;
}

#[command]
pub fn ntm(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
    let _ = msg.channel_id.say(&ctx, "Mange Tes Morts");
    Ok(())
}
