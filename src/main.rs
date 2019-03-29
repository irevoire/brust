#[macro_use]
extern crate serenity;

mod commands;

use std::{collections::HashMap, fs::File, io::Read, path::Path};

use serenity::{
    framework::standard::{help_commands, StandardFramework},
    model::gateway::Ready,
    prelude::*,
};

struct Score;

impl TypeMapKey for Score {
    type Value = HashMap<String, i64>;
}

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

const DISCORD_TOKEN: &str = "LALALA";

fn parse() -> HashMap<String, i64> {
    let mut hash: HashMap<String, i64> = HashMap::default();
    let path = Path::new("/tmp/hello.txt");

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open file {}", why),
        Ok(file) => file,
    };

    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    for line in s.split("\n") {
        if line == "" {
            break;
        }
        let mut split = line.split(" ");
        let name = split
            .next()
            .unwrap_or_else(|| panic!(format!("can't parse this line: {}", line)));
        let nb: i64 = split
            .next()
            .unwrap_or_else(|| panic!(format!("need a score: {}", line)))
            .parse()
            .unwrap_or_else(|_| panic!(format!("can't parse this score: {}", line)));
        hash.insert(name.to_string(), nb);
    }
    return hash;
}

fn main() {
    let score = parse();
    let token = DISCORD_TOKEN;
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    {
        let mut data = client.data.lock();
        data.insert::<Score>(score);
    }

    client.with_framework(
        StandardFramework::new()
            .configure(|c| {
                c.allow_whitespace(true)
                    .on_mention(true)
                    .prefix("!")
                    //.prefix_only_cmd(help)
                    .delimiters(vec![", ", ",", " "])
            })
            .before(|_, msg, command_name| {
                println!(
                    "Got command '{}' by user '{}'",
                    command_name, msg.author.name
                );
                true // if `before` returns false, command processing doesn't happen.
            })
            .after(|_, _, command_name, error| match error {
                Ok(()) => println!("Processed command '{}'", command_name),
                Err(why) => println!("Command '{}' returned error {:?}", command_name, why),
            })
            .customised_help(help_commands::with_embeds, |c| {
                c.individual_command_tip("🦀 Bonjour 🦀")
                    .max_levenshtein_distance(5)
                    // `{}` refers to a command's name.
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
