

use serenity::framework::standard::{macros::command, Args, CommandResult};

use serenity::{
    model::channel::Message,
    prelude::{Context},
};

#[command]
pub fn mock(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
    let mut new = String::new();
    let mut last = false;
    for c in msg.content.chars().skip("!mock".len()) {
        if !c.is_ascii_alphabetic() {
            new.push(c);
            continue;
        }
        if last {
            new.push(c.to_ascii_lowercase());
        } else {
            new.push(c.to_ascii_uppercase());
        }
        last = !last;
    }
    let _ = msg.reply(&ctx, new);
    let _ = msg.delete(&ctx);

    Ok(())
}
