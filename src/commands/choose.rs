use rand::thread_rng;
use rand::Rng;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::{model::channel::Message, prelude::Context};

#[command]
#[description = r#"Choose an argument randomly"#]
pub fn choose(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let nb_choice = args.advance().remaining();
    if nb_choice == 0 {
        let _ = msg.reply(&ctx, "Give me arguments");
        return Ok(());
    }
    let mut rng = thread_rng();
    let choice = rng.gen_range(0, nb_choice);

    let _ = msg.reply(&ctx, args.iter::<String>().nth(choice).unwrap().unwrap());
    Ok(())
}
