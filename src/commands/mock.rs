use serenity::framework::standard::{macros::command, Args, CommandResult};

use serenity::{model::channel::Message, prelude::Context};

#[command]
pub fn mock(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
    let mut new = String::new();
    let mut last = false;
    for c in msg.content.chars().skip("!mock".chars().count()) {
        if !c.is_alphabetic() {
            new.push(c);
            continue;
        }
        if last {
            new.push_str(&c.to_lowercase().to_string());
        } else {
            new.push_str(&c.to_uppercase().to_string());
        }
        last = !last;
    }
    let _ = msg.reply(&ctx, new);
    let _ = msg.delete(&ctx);

    Ok(())
}
