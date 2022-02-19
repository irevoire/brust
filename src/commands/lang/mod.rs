use anyhow::anyhow;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult};
use serenity::http::Typing;
use serenity::{model::channel::Message, prelude::Context};

mod c;
mod cpp;
mod go;
mod java;
mod kotlin;
mod node;
mod php;
mod python;
mod r;
mod ruby;
mod rust;
mod swift;

use c::*;
use cpp::*;
use go::*;
use java::*;
use kotlin::*;
use node::*;
use php::*;
use python::*;
use r::*;
use ruby::*;
use rust::*;
use swift::*;

#[group]
#[commands(run, rust, python, ruby, node, go, swift, kotlin, php, r, c, cpp, java)]
struct Lang;

#[command]
#[usage("```language\ncode\n```")]
#[example("```rust\nprintln!(\"{}\", 2 + 2)\n```")]
#[description = r#"Try to guess your language by the type of codeblock you used and execute the code in the playground."#]
pub async fn run(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let typing = Typing::start(ctx.http.clone(), *msg.channel_id.as_u64())?;

    let block = args.rest().trim_matches(|c| c != '`').trim_matches('`');
    let (lang, code) = block
        .split_once(char::is_whitespace)
        .ok_or_else(|| anyhow!("Could not detect the language"))?;

    let res = match lang {
        "rust" | "rs" => {
            if code.contains("fn main(") {
                execute_rust(code).await?
            } else {
                execute_rust(&format!(
                    "fn main() -> anyhow::Result<()> {{\n{code}\nOk(())\n}}"
                ))
                .await?
            }
        }
        "golang" | "go" => {
            if code.contains("func main(") {
                execute_go(code).await?
            } else {
                execute_rust(&format!(
                    "package main
import \"fmt\"
func main() {{
    {code}
}}",
                ))
                .await?
            }
        }
        "ruby" | "rb" => execute_ruby(code).await?,
        "python" | "py" => execute_python(code).await?,
        "kotlin" | "kt" => execute_kotlin(code).await?,
        "c++" | "cpp" => execute_cpp(code).await?,
        "js" | "javascript" => execute_node(code).await?,
        "r" => execute_r(code).await?,
        "c" => execute_c(code).await?,
        "php" => execute_php(code).await?,
        "swift" => execute_swift(code).await?,
        "java" => execute_java(code).await?,
        lang => Err(anyhow!("{lang} not handled"))?,
    };

    msg.reply(
        &ctx,
        format!("```{}```", res.chars().take(2000 - 8).collect::<String>()),
    )
    .await?;

    typing.stop();

    Ok(())
}
