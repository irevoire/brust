#[macro_use]
extern crate serenity;

mod commands;
mod hook;

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
        hook::pun::init();
    }
}

fn main() {
    kankyo::load().expect("Failed to load .env file");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let mut client = Client::new(&token, Handler).expect("Err creating client");
    let filename = env::var("SCORE_FILE").expect("Expected a file to save the score");

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
                    .striked_commands_tip(None)
            })
            .group("Tribunal", |g| {
                g.desc("Commande du tribunal des blagues")
                    .command("nul", |c| c.cmd(commands::blague::nul).desc("Blague nulle"))
                    .command("mdr", |c| c.cmd(commands::blague::mdr).desc("Bonne blague"))
                    .command("blague", |c| {
                        c.cmd(commands::blague::blague)
                            .desc("Affiche le score des blagues")
                    })
            }),
    );

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
