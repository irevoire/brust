mod commands;

use commands::{mock::*, tg::*};
use serenity::{
    framework::{standard::macros::group, StandardFramework},
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

#[group]
#[commands(tg, mock)]
struct General;

fn main() {
    kankyo::load(false).expect("Failed to load .env file");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let mut client = Client::new(&token, Handler).expect("Err creating client");
    {
        let mut data = client.data.write();
        data.insert::<commands::tg::Tg>(commands::tg::init());
    }

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("!").delimiters(vec![", ", ",", " "]))
            .group(&GENERAL_GROUP),
    );

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
