mod commands;
mod imgflip;
mod utils;

use commands::*;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::{
        standard::macros::{help, hook},
        standard::{help_commands, Args, CommandGroup, CommandResult, HelpOptions},
        StandardFramework,
    },
    http::Http,
    model::gateway::Ready,
    model::interactions::{
        application_command::{ApplicationCommand, ApplicationCommandOptionType},
        Interaction,
    },
    model::prelude::*,
    prelude::*,
};
use std::collections::HashSet;
use std::env;
use std::sync::Arc;

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        /*
        TODO: buy a better server which handle SIMD
        ApplicationCommand::create_global_application_command(&ctx, |command| {
            command
                .name("uwu")
                .description("uωu")
                .create_option(|option| {
                    option
                        .name("owo")
                        .description("youw text")
                        .kind(ApplicationCommandOptionType::String)
                        .required(true)
                })
        })
        .await
        .unwrap();
        */

        ApplicationCommand::create_global_application_command(&ctx, |command| {
            command
                .name("big")
                .description("Makes your text 🅱️  🇮 🇬")
                .create_option(|option| {
                    option
                        .name("text")
                        .description("your text")
                        .kind(ApplicationCommandOptionType::String)
                        .required(true)
                })
        })
        .await
        .unwrap();

        for command in ApplicationCommand::get_global_application_commands(&ctx)
            .await
            .unwrap()
        {
            // if ["uwu", "big"].contains(&command.name.as_ref()) {
            if ["big"].contains(&command.name.as_ref()) {
                continue;
            }
            ApplicationCommand::delete_global_application_command(&ctx, command.id)
                .await
                .unwrap();
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let command_name = command.data.name.to_string();
            let result = match command_name.as_str() {
                // "uwu" => self.handle_uwuify(ctx, command).await,
                "big" => self.handle_big(ctx, command).await,
                _ => {
                    println!("unknown interaction");
                    Ok(())
                }
            };
            if let Err(e) = result {
                println!("Unexpected error with command {}:\n{}", command_name, e);
            }
        }
    }
}

pub struct Random;

impl TypeMapKey for Random {
    type Value = Arc<Mutex<SmallRng>>;
}

#[help]
#[individual_command_tip = ":crab: To get help with an individual command, pass its name as an argument to this command. :crab:"]
#[wrong_channel = "Hide"]
#[max_levenshtein_distance(3)]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[hook]
async fn after(ctx: &Context, msg: &Message, command_name: &str, command_result: CommandResult) {
    if let Err(why) = command_result {
        let speech_bubble_emoji = "💬".parse::<ReactionType>().unwrap();

        eprintln!("Command '{}' returned error {:?}", command_name, why);

        let _err = msg.react(ctx, speech_bubble_emoji.clone()).await;

        let tmp_emoji = speech_bubble_emoji.clone();

        let want_error_msg = msg
            .await_reaction(ctx)
            .timeout(std::time::Duration::from_secs(60 * 10))
            .filter(move |reaction| reaction.emoji == tmp_emoji)
            .await;
        let _err = msg.delete_reaction_emoji(ctx, speech_bubble_emoji).await;
        if want_error_msg.is_some() {
            let _ = msg.reply(&ctx, why).await;
        }
    }
}

#[tokio::main]
async fn main() {
    kankyo::load(false).expect("Failed to load .env file");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let http = Http::new_with_token(&token);

    // We will fetch your bot's owners and id
    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| {
            c.prefix("!")
                .on_mention(Some(bot_id))
                .owners(owners)
                .delimiters(vec![", ", ",", " "])
        })
        .group(&GENERAL_GROUP)
        .group(&CUTE_GROUP)
        .after(after)
        .help(&MY_HELP);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .application_id(bot_id.0)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }
    let mut rng = SmallRng::from_entropy();

    {
        let mut data = client.data.write().await;
        data.insert::<commands::Tg>(Arc::new(Mutex::new(commands::init_tg(&mut rng))));
        data.insert::<Random>(Arc::new(Mutex::new(rng)));
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}
