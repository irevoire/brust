use crate::utils::unicode_to_safe_ascii;
use anyhow::Result;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::{Message, ReactionType, UserId};
use serenity::prelude::Context;
use std::collections::{HashMap, HashSet};

#[command]
#[description = r#"React to a message.
You can:
    - Type your reaction right after the `!react` to react to the previous message
    - @someone and write your reaction right after
    - Write nothing and go fuck yourself"#]
pub async fn react(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let message;
    match args.single::<UserId>() {
        Ok(id) => {
            let tmp = msg
                .channel_id
                .messages(&ctx, |retriever| retriever.before(msg.id))
                .await?
                .iter()
                .find(|msg| msg.author.id == id)
                .cloned();
            if tmp.is_none() {
                args.restore();
                message = get_last_message(ctx, msg).await?;
            } else {
                message = tmp.unwrap();
            }
        }
        Err(_) => {
            message = msg
                .channel_id
                .messages(&ctx, |retriever| retriever.before(msg.id).limit(1))
                .await?[0]
                .clone();
        }
    };
    let rest = args.rest();

    let mut already_used_emoji = HashSet::new();
    // get all the already used emoji
    for reaction in message.reactions.iter() {
        match &reaction.reaction_type {
            ReactionType::Unicode(e) => already_used_emoji.insert(e.clone()),
            _ => false, // useless
        };
    }

    for c in rest.chars() {
        let emoji = match char_to_emoji(c, &already_used_emoji) {
            Some(c) => c,
            None => {
                msg.react(&ctx, '🇵').await?;
                msg.react(&ctx, '🇩').await?;
                return Ok(());
            }
        };
        message
            // TODO: does this « cast » to char really work with all emoji?
            .react(&ctx, emoji.clone().chars().next().unwrap())
            .await?;
        already_used_emoji.insert(emoji);
    }
    msg.delete(&ctx).await?;
    Ok(())
}

async fn get_last_message(ctx: &Context, msg: &Message) -> Result<Message> {
    Ok(msg
        .channel_id
        .messages(&ctx, |retriever| retriever.before(msg.id).limit(1))
        .await?[0]
        .clone())
}

fn char_to_emoji(c: char, banned_emoji: &HashSet<String>) -> Option<String> {
    let base = generate_equivalence(); // we hope rustc will optimize all this shit until const fn get stabilized
    let c = match unicode_to_safe_ascii(c) {
        None => return None,
        Some(c) => c,
    };

    let equivalence = base.get(&c);
    if equivalence.is_none() {
        return None;
    }
    let equivalence = equivalence.unwrap();
    for emoji in equivalence {
        if !banned_emoji.contains(emoji) {
            return Some(emoji.clone());
        }
    }
    None
}

fn generate_equivalence() -> HashMap<char, Vec<String>> {
    let mut base = HashMap::new();
    base.insert(
        'a',
        vec!["🇦".to_string(), "🅰️".to_string(), "🔼".to_string()],
    );
    base.insert('b', vec!["🇧".to_string(), "🅱️".to_string()]);
    base.insert(
        'c',
        vec![
            "🇨".to_string(),
            "↪️".to_string(),
            "☪️".to_string(),
            "🗜️".to_string(),
        ],
    );
    base.insert('d', vec!["🇩".to_string(), "↩️".to_string(), "▶️".to_string()]);
    base.insert('e', vec!["🇪".to_string(), "3️⃣".to_string()]);
    base.insert('f', vec!["🇫".to_string()]);
    base.insert('g', vec!["🇬".to_string()]);
    base.insert('h', vec!["🇭".to_string(), "♓".to_string()]);
    base.insert(
        'i',
        vec![
            "🇮".to_string(),
            "ℹ️".to_string(),
            "❕".to_string(),
            "📍".to_string(),
            "💈".to_string(),
        ],
    );
    base.insert('j', vec!["🇯".to_string(), "⤴️".to_string()]);
    base.insert('k', vec!["🇰".to_string()]);
    base.insert('l', vec!["🇱".to_string()]);
    base.insert(
        'm',
        vec![
            "🇲".to_string(),
            "Ⓜ️".to_string(),
            "♏".to_string(),
            "♍".to_string(),
        ],
    );
    base.insert('n', vec!["🇳".to_string(), "♑".to_string()]);
    base.insert(
        'o',
        vec![
            "🇴".to_string(),
            "🅾️".to_string(),
            "🔄".to_string(),
            "🔁".to_string(),
            "0️⃣".to_string(),
            "🔃".to_string(),
            "🔵".to_string(),
            "💿".to_string(),
            "🔘".to_string(),
            "⚙️".to_string(),
        ],
    );
    base.insert('p', vec!["🇵".to_string(), "🅿️".to_string()]);
    base.insert('q', vec!["🇶".to_string()]);
    base.insert('r', vec!["🇷".to_string()]);
    base.insert('s', vec!["🇸".to_string(), "5️⃣".to_string()]);
    base.insert('t', vec!["🇹".to_string(), "✝️".to_string(), "⬆️".to_string()]);
    base.insert(
        'u',
        vec![
            "🇺".to_string(),
            "⛎".to_string(),
            "🇻".to_string(),
            "♈".to_string(),
        ],
    );
    base.insert(
        'v',
        vec![
            "🇻".to_string(),
            "♈".to_string(),
            "🇺".to_string(),
            "⛎".to_string(),
        ],
    );
    base.insert('w', vec!["🇼".to_string()]);
    base.insert(
        'x',
        vec![
            "🇽".to_string(),
            "🔀".to_string(),
            "❌".to_string(),
            "✖️".to_string(),
        ],
    );
    base.insert('y', vec!["🇾".to_string()]);
    base.insert('z', vec!["🇿".to_string(), "2️⃣".to_string()]);
    base.insert(
        '0',
        vec![
            "0️⃣".to_string(),
            "0️⃣".to_string(),
            "🔃".to_string(),
            "🅾️".to_string(),
            "🇴".to_string(),
            "🔄".to_string(),
            "🔁".to_string(),
            "🅱️".to_string(),
        ],
    );
    base.insert('1', vec!["1️⃣".to_string()]);
    base.insert('2', vec!["2️⃣".to_string()]);
    base.insert('3', vec!["3️⃣".to_string()]);
    base.insert('4', vec!["4️⃣".to_string()]);
    base.insert('5', vec!["5️⃣".to_string()]);
    base.insert('6', vec!["6️⃣".to_string()]);
    base.insert('7', vec!["7️⃣".to_string()]);
    base.insert('8', vec!["8️⃣".to_string()]);
    base.insert('9', vec!["9️⃣".to_string()]);
    base.insert(
        ' ',
        vec![
            "▪️".to_string(),
            "◾".to_string(),
            "◼️".to_string(),
            "⬛".to_string(),
            "➖".to_string(),
        ],
    );
    base.insert(
        '.',
        vec![
            "⏺️".to_string(),
            "🔹".to_string(),
            "🔘".to_string(),
            "🔵".to_string(),
            "🔴".to_string(),
            "🟣".to_string(),
            "🟢".to_string(),
            "🟡".to_string(),
        ],
    );
    base.insert(
        '!',
        vec!["❗".to_string(), "❕".to_string(), "‼️".to_string()],
    );
    base.insert(
        '?',
        vec!["❓".to_string(), "❔".to_string(), "⁉️".to_string()],
    );
    base.insert('#', vec!["#️⃣".to_string()]);
    base.insert('*', vec!["*️⃣".to_string()]);

    base
}
