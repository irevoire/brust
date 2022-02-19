use anyhow::{anyhow, Result};
use serde_json::json;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::http::Typing;
use serenity::{model::channel::Message, prelude::Context};

#[command]
#[usage("[php expression]")]
#[example("2 + 2")]
#[description = r#"Run php code in the following playground: https://code.sololearn.com/
Your code will be written in a `print`.
For example if you write `!php 2 + 2` it'll actually execute:
```php
<?php print("2 + 2"); ?>
```"#]
pub async fn php(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let typing = Typing::start(ctx.http.clone(), *msg.channel_id.as_u64())?;

    let code = format!("print(\"{}\");", args.rest());

    let res = execute_php(&code).await?;
    let res = format!("```\n{}\n```", res);
    msg.reply(&ctx, res).await?;

    typing.stop();

    Ok(())
}

pub async fn execute_php(code: &str) -> Result<String> {
    let request = json!({
      "code": format!("<?php {} ?>", code),
      "language": "php",
      "input": "",
      "codeId": null
    });
    let res: serde_json::Value = reqwest::Client::new()
        .post("https://api2.sololearn.com/v2/codeplayground/v2/compile")
        .json(&request)
        .send()
        .await?
        .json()
        .await?;

    if res["success"] == json!(true) {
        Ok(res["data"]["output"].as_str().unwrap().to_string())
    } else {
        Err(anyhow!("Unknown error"))?
    }
}
