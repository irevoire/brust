use anyhow::{anyhow, Result};
use serde_json::json;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::http::Typing;
use serenity::{model::channel::Message, prelude::Context};

#[command]
#[usage("[go expression]")]
#[example("2 + 2")]
#[description = r#"Run rust code in the following playground: https://code.sololearn.com/cUs4dp6t5jiq 
Your code will be prefixed by a `p`.
For example if you write `!go 2 + 2` it'll actually execute:
```go
package main
import "fmt"
func main() {
    fmt.Println(2 + 2)
}
```"#]
pub async fn go(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let typing = Typing::start(ctx.http.clone(), *msg.channel_id.as_u64())?;

    let code = format!(
        "package main
import \"fmt\"
func main() {{
    fmt.Println({})
}}
",
        args.rest()
    );

    let res = execute_go(&code).await?;
    let res = format!("```\n{}\n```", res);
    msg.reply(&ctx, res).await?;

    typing.stop();

    Ok(())
}

pub async fn execute_go(code: &str) -> Result<String> {
    let request = json!({
      "code": code,
      "language": "go",
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
