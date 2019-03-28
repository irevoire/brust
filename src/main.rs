#[macro_use]
extern crate serenity;

use std::{collections::HashMap, env, fmt::Write, sync::Arc};

use serenity::prelude::*;
use serenity::{
    client::bridge::gateway::{ShardId, ShardManager},
    framework::standard::{
        help_commands, Args, CommandOptions, DispatchError, HelpBehaviour, StandardFramework,
    },
    model::{channel::Message, gateway::Ready, Permissions},
    prelude::*,
    utils::{content_safe, ContentSafeOptions},
};

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct CommandCounter;

impl TypeMapKey for CommandCounter {
    type Value = HashMap<String, u64>;
}

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

    {
        let mut data = client.data.lock();
        data.insert::<CommandCounter>(HashMap::default());
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    client.with_framework(
        StandardFramework::new()
            .configure(|c| {
                c.allow_whitespace(true)
                    .on_mention(true)
                    .prefix("~")
                    //.prefix_only_cmd(help)
                    .delimiters(vec![", ", ",", " "])
            })
            .before(|ctx, msg, command_name| {
                println!(
                    "Got command '{}' by user '{}'",
                    command_name, msg.author.name
                );

                let mut data = ctx.data.lock();
                let counter = data
                    .get_mut::<CommandCounter>()
                    .expect("Expected CommandCounter in ShareMap.");
                let entry = counter.entry(command_name.to_string()).or_insert(0);
                *entry += 1;

                true // if `before` returns false, command processing doesn't happen.
            })
            .after(|_, _, command_name, error| match error {
                Ok(()) => println!("Processed command '{}'", command_name),
                Err(why) => println!("Command '{}' returned error {:?}", command_name, why),
            })
            //.unrecognised_command(help)
            //.message_without_command(help)
            .on_dispatch_error(|_ctx, msg, error| {
                if let DispatchError::RateLimited(seconds) = error {
                    let _ = msg
                        .channel_id
                        .say(&format!("Try this again in {} seconds.", seconds));
                }
            })
            .customised_help(help_commands::with_embeds, |c| {
                c.individual_command_tip("🦀 Bonjour 🦀")
                    .max_levenshtein_distance(5)
                    // `{}` refers to a command's name.
                    .command_not_found_text("Could not find: `{}`.")
            })
            .command("commands", |c| c.cmd(commands))
            .command("say", |c| c.cmd(say))
            .command("latency", |c| c.cmd(latency))
            .command("nul", |c| c.cmd(nul)),
    );

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}

/*
fn add(user: String) {
    let path = Path::new("/tmp/hello.txt");

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open file {}", why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }
    for line in s.split("\n") {
        if line.starts_with(user) {
            let number = line.split(" ").last();
            let nb: u64 = number.parse();
            number = format!("{}", nb + 1);
        }
    }
    println!("{}", s);
}
*/

command!(nul(ctx, msg, args) {
    if args.len() != 1 {
        msg.reply("Donne moi **un** nom !");
        return Ok(());
    }
    let user: String = args.iter().next().unwrap().unwrap();
    if !user.starts_with("<@") || !user.ends_with(">") {
        msg.reply("Le nom est malformé !");
        return Ok(());
    }

    let user_id: u64 = match user[2..user.len() - 1].parse::<u64>() {
        Err(_) => {
            msg.reply("Le nom est malformé !");
            return Ok(());
        },
        Ok(n) => n,
    };

    // println!("{:?}", ctx);
    let user = serenity::model::id::UserId(user_id);
    let user = match user.get() {
        Err(_) => {
            msg.reply("Utilisateur inconnu");
            return Ok(());
        },
        Ok(u) => u,
    };

    if let Err(why) = msg.channel_id.say(format!("Shame {}!", user)) {
        println!("Error sending message: {:?}", why);
    }
});

command!(commands(ctx, msg, _args) {
    let mut contents = "Commands used:\n".to_string();

    let data = ctx.data.lock();
    let counter = data.get::<CommandCounter>().expect("Expected CommandCounter in ShareMap.");

    for (k, v) in counter {
        let _ = write!(contents, "- {name}: {amount}\n", name=k, amount=v);
    }

    if let Err(why) = msg.channel_id.say(&contents) {
        println!("Error sending message: {:?}", why);
    }
});

command!(say(_ctx, msg, args) {
    let mut settings = if let Some(guild_id) = msg.guild_id {
        ContentSafeOptions::default()
            .clean_channel(false)
            .display_as_member_from(guild_id)
    } else {
        ContentSafeOptions::default()
            .clean_channel(false)
            .clean_role(false)
    };

    let mut content = content_safe(&args.full(), &settings);

    if let Err(why) = msg.channel_id.say(&content) {
        println!("Error sending message: {:?}", why);
    }
});

command!(latency(ctx, msg, _args) {
    let data = ctx.data.lock();

    let shard_manager = match data.get::<ShardManagerContainer>() {
        Some(v) => v,
        None => {
            let _ = msg.reply("There was a problem getting the shard manager");

            return Ok(());
        },
    };

    let manager = shard_manager.lock();
    let runners = manager.runners.lock();

    let runner = match runners.get(&ShardId(ctx.shard_id)) {
        Some(runner) => runner,
        None => {
            let _ = msg.reply("No shard found");

            return Ok(());
        },
    };

    let _ = msg.reply(&format!("The shard latency is {:?}", runner.latency));
});
