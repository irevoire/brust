use rand::Rng;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::{model::channel::Message, prelude::Context};

#[command]
#[description = r#"Choose an argument randomly"#]
pub async fn choose(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;

    let nb_choice = args.remaining();
    if nb_choice == 0 {
        let _ = msg.reply(&ctx, "Give me arguments").await?;
        return Ok(());
    }
    let mut rng = data.get::<crate::Random>().unwrap().lock().await;
    let choice = rng.gen_range(0..nb_choice);

    let _ = msg
        .reply(&ctx, args.iter::<String>().nth(choice).unwrap().unwrap())
        .await?;
    Ok(())
}
