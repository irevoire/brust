use anyhow::anyhow;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult};
use serenity::http::Typing;
use serenity::{model::channel::Message, prelude::Context};

mod c;
mod cpp;
mod cs;
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
use cs::*;
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
#[commands(
    run, rust, python, ruby, node, go, swift, kotlin, php, r, c, cpp, cs, java
)]
struct Lang;

#[command]
#[usage("```language\ncode\n```")]
#[example("```rust\nprintln!(\"{}\", 2 + 2)\n```")]
#[description = r#"Try to guess your language by the type of codeblock you used and execute the code in a playground."#]
pub async fn run(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let typing = Typing::start(ctx.http.clone(), *msg.channel_id.as_u64())?;

    let block = args.rest().trim_matches(|c| c != '`').trim_matches('`');
    let (lang, code) = block
        .split_once(char::is_whitespace)
        .ok_or_else(|| anyhow!("Could not detect the language"))?;

    let res = match lang.to_lowercase().as_str() {
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
                execute_go(&format!(
                    "package main
import \"fmt\"
func main() {{
    {code}
}}",
                ))
                .await?
            }
        }
        "c#" | "cs" => {
            if code.contains("static void Main(") {
                execute_cs(code).await?
            } else {
                execute_cs(&format!(
                    "namespace Run
{{
    class Hello {{
        static void Main(string[] args)
        {{
            {code}
        }}
    }}
}}",
                ))
                .await?
            }
        }
        "kotlin" | "kt" => {
            if code.contains("fun main(") {
                execute_kotlin(code).await?
            } else {
                execute_kotlin(&format!("fun main(args : Array<String>) {{ {code} }}",)).await?
            }
        }
        "c++" | "cpp" => {
            if code.contains("main(") {
                execute_cpp(code).await?
            } else {
                execute_cpp(&format!(
                    "
#include <iostream>
using namespace std;

int main() {{
    {code}
    return 0;
}}",
                ))
                .await?
            }
        }
        "c" => {
            if code.contains("main(") {
                execute_c(code).await?
            } else {
                execute_c(&format!(
                    "#include <stdio.h>

int main() {{
    {code}
    return 0;
}}",
                ))
                .await?
            }
        }
        "java" => {
            if code.contains("public static void main(") {
                execute_java(code).await?
            } else {
                execute_java(&format!(
                    "public class Program
{{
    public static void main(String[] args) {{
		{code}
	}}
}}
",
                ))
                .await?
            }
        }
        "ruby" | "rb" => execute_ruby(code).await?,
        "python" | "py" => execute_python(code).await?,
        "js" | "javascript" => execute_node(code).await?,
        "r" => execute_r(code).await?,
        "php" => execute_php(code).await?,
        "swift" => execute_swift(code).await?,
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
