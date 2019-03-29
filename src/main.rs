#[macro_use]
extern crate serenity;

use std::{collections::HashMap, fmt::Write, fs::File, io::Read, path::Path, sync::Arc};

use serenity::{
    client::bridge::gateway::{ShardId, ShardManager},
    framework::standard::{help_commands, DispatchError, StandardFramework},
    model::gateway::Ready,
    prelude::*,
};

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct CommandCounter;

impl TypeMapKey for CommandCounter {
    type Value = HashMap<String, u64>;
}

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
        data.insert::<CommandCounter>(HashMap::default());
        data.insert::<Score>(score);
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
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
            .command("latency", |c| c.cmd(latency))
            .command("nul", |c| c.cmd(nul))
            .command("mdr", |c| c.cmd(nul))
            .command("blague", |c| c.cmd(blague)),
    );

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}

command!(blague(ctx, msg, _args) {
    let mut res = "Blagues :\n".to_string();

    let mut data = ctx.data.lock();
    let scores = data
        .get::<Score>()
        .expect("Expected Score in ShareMap.");

    for (k, v) in scores {
        let _ = write!(res, "- {}: {}\n", k, v);
    }

    if let Err(why) = msg.channel_id.say(&res) {
        println!("Error sending message: {:?}", why);
    }
});

fn get_user_id(user: String) -> Result<String, String> {
    println!("user: {}", user);
    let start;
    if !user.starts_with("<@") || !user.ends_with(">") {
        return Err("Le nom est mal formé !".to_string());
    }
    if user.starts_with("<@!") {
        start = 3
    } else {
        start = 2
    }

    let user_id: u64 = match user[start..user.len() - 1].parse() {
        Err(_) => return Err("Le nom est mal formé !".to_string()),
        Ok(n) => n,
    };

    let user = serenity::model::id::UserId(user_id);
    return match user.to_user() {
        Err(_) => Err("Utilisateur inconnu".to_string()),
        Ok(u) => Ok(u.to_string()),
    };
}

command!(mdr(ctx, msg, args) {
    if args.len() != 1 {
        let _ = msg.reply("Donne moi **un** nom !");
        let _ = msg.react('❎');
        return Ok(());
    }
    let user: String = args.iter().next().unwrap().unwrap();
    let user = match get_user_id(user) {
        Err(e) => {
            let _ = msg.reply(&e);
            let _ = msg.react('❎');
            return Ok(());
        },
        Ok(u) => u,
    };
    let mut data = ctx.data.lock();
    let score = data
        .get_mut::<Score>()
        .expect("Expected Score in ShareMap.");
    let entry = score.entry(user.to_string()).or_insert(0);
    *entry += 1;
    let _ = msg.react('👌');
});

command!(nul(ctx, msg, args) {
    if args.len() != 1 {
        let _ = msg.reply("Donne moi **un** nom !");
        let _ = msg.react('❎');
        return Ok(());
    }
    let user: String = args.iter().next().unwrap().unwrap();
    let user = match get_user_id(user) {
        Err(e) => {
            let _ = msg.reply(&e);
            let _ = msg.react('❎');
            return Ok(());
        },
        Ok(u) => u,
    };
    let mut data = ctx.data.lock();
    let score = data
        .get_mut::<Score>()
        .expect("Expected Score in ShareMap.");
    let entry = score.entry(user.to_string()).or_insert(0);
    *entry -= 1;
    let _ = msg.react('👌');
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
