use crate::Score;
use std::fmt::Write;

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
