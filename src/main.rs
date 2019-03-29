#[macro_use]
extern crate serenity;

mod commands;

use serenity::{
    framework::standard::{help_commands, StandardFramework},
    model::gateway::Ready,
    prelude::*,
};

use std::env;

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

const DISCORD_TOKEN: &str = "LALALA";

fn main() {
    let token = DISCORD_TOKEN;
    let mut client = Client::new(&token, Handler).expect("Err creating client");
    let filename = env::args().skip(1).next().unwrap();

    {
        let mut data = client.data.lock();
        data.insert::<commands::blague::Score>(commands::blague::init(&filename));
        data.insert::<commands::blague::FileScore>(filename);
    }

    client.with_framework(
        StandardFramework::new()
            .configure(|c| {
                c.allow_whitespace(true)
                    .on_mention(true)
                    .prefix("!")
                    .delimiters(vec![", ", ",", " "])
            })
            .customised_help(help_commands::with_embeds, |c| {
                c.individual_command_tip("🦀 Bonjour 🦀")
                    .max_levenshtein_distance(5)
                    .command_not_found_text("Could not find: `{}`.")
            })
            .command("nul", |c| c.cmd(commands::blague::nul))
            .command("mdr", |c| c.cmd(commands::blague::mdr))
            .command("blague", |c| c.cmd(commands::blague::blague)),
    );

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
