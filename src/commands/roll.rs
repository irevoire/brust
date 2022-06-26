use anyhow::anyhow;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::{model::channel::Message, prelude::Context};

#[command]
#[description = r#"Roll dice"#]
pub async fn roll(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    std::env::set_var("NO_COLOR", "true");

    let data = ctx.data.read().await;
    let mut rng = data.get::<crate::Random>().unwrap().lock().await;

    let source = args.rest();
    let res = dicey::Interpreter::run_with_rng(source, &mut *rng);
    drop(rng);
    let res = match res {
        Err(dicey::Error::Parser(error)) => Err(anyhow!("```\n{}\n```", error.to_report()))?,
        Err(e) => format!("{e}"),
        Ok(res) => res.to_string(),
    };

    msg.reply(&ctx, res).await?;
    Ok(())
}
