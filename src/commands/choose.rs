use rand::Rng;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::{model::channel::Message, prelude::Context};

#[command]
#[description = r#"Choose an argument randomly"#]
pub fn choose(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read();

    let nb_choice = args.remaining();
    if nb_choice == 0 {
        let _ = msg.reply(&ctx, "Give me arguments");
        return Ok(());
    }
    let mut rng = data.get::<crate::Random>().unwrap().lock().unwrap();
    let choice = rng.gen_range(0, nb_choice);

    let _ = msg.reply(&ctx, args.iter::<String>().nth(choice).unwrap().unwrap());
    Ok(())
}
