mod commands;
mod imgflip;
mod utils;

use commands::*;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use serenity::{
    framework::{
        standard::macros::{group, help},
        standard::{help_commands, Args, CommandGroup, CommandResult, HelpOptions},
        StandardFramework,
    },
    model::gateway::Ready,
    model::prelude::*,
    prelude::*,
};
use std::collections::HashSet;
use std::env;
use std::sync::Mutex;

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

pub struct Random;

impl TypeMapKey for Random {
    type Value = Mutex<SmallRng>;
}

#[help]
#[individual_command_tip = ":crab: To get help with an individual command, pass its name as an argument to this command. :crab:"]
#[wrong_channel = "Hide"]
#[max_levenshtein_distance(3)]
fn my_help(
    context: &mut Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    help_commands::with_embeds(context, msg, args, help_options, groups, owners)
}

#[group]
#[commands(tg, mock, roasted, big, react, choose)]
struct General;

#[group]
#[commands(cat, dog)]
struct Cute;

fn main() {
    kankyo::load(false).expect("Failed to load .env file");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let mut client = Client::new(&token, Handler).expect("Err creating client");
    let mut rng = SmallRng::from_entropy();

    {
        let mut data = client.data.write();
        data.insert::<commands::Tg>(Mutex::new(commands::init_tg(&mut rng)));
        data.insert::<Random>(Mutex::new(rng));
    }

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("!").delimiters(vec![", ", ",", " "]))
            .group(&GENERAL_GROUP)
            .group(&CUTE_GROUP)
            .help(&MY_HELP),
    );

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
