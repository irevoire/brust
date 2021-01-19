use rand::Rng;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::{model::channel::Message, prelude::Context};

#[command]
#[description = r#"Choose an argument randomly"#]
#[usage("[List of things you want to choose from]+")]
#[example("taco pizza")]
pub async fn choose(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;

    let nb_choice = args.remaining();
    if nb_choice == 0 {
        Err(anyhow::anyhow!("Give me arguments"))?;
    }
    let mut rng = data.get::<crate::Random>().unwrap().lock().await;
    let choice = rng.gen_range(0..nb_choice);

    msg.reply(&ctx, args.iter::<String>().nth(choice).unwrap().unwrap())
        .await?;
    Ok(())
}
