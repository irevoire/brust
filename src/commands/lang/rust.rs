use anyhow::{anyhow, Result};
use serde_json::json;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::http::Typing;
use serenity::{model::channel::Message, prelude::Context};

#[command]
#[usage("[rust expression]")]
#[example("2 + 2")]
#[description = r#"Run rust code in the rust playground: https://play.rust-lang.org/
Your code will be embedded in a block that'll be printed.
For example if you write `!rust 2 + 2` it'll actually execute:
```rust
fn main() {
    println!("{:?}", { 2 + 2 });
}
```
You can see the complete list of available crates here: https://github.com/integer32llc/rust-playground/blob/master/compiler/base/Cargo.toml"#]
pub async fn rust(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let typing = Typing::start(ctx.http.clone(), *msg.channel_id.as_u64())?;

    let code = format!(
        "fn main() {{ println!(\"{{:?}}\", {{ {} }}); }}",
        args.rest()
    );

    let res = execute_rust(&code).await?;
    let res = format!("```\n{}\n```", res);
    msg.reply(&ctx, res).await?;

    typing.stop();

    Ok(())
}

pub async fn execute_rust(code: &str) -> Result<String> {
    let request = json!({
        "channel": "nightly",
        "mode": "debug",
        "edition": "2021",
        "crateType": "bin",
        "tests": false,
        "backtrace": false,
        "code": code,
    });
    let res: serde_json::Value = reqwest::Client::new()
        .post("https://play.rust-lang.org/execute")
        .json(&request)
        .send()
        .await?
        .json()
        .await?;

    if res["success"] == json!(true) {
        Ok(res["stdout"].as_str().unwrap().to_string())
    } else if res["success"] == json!(false) {
        Ok(res["stderr"].as_str().unwrap().to_string())
    } else {
        Err(anyhow!("Unknown error"))?
    }
}
