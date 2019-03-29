use serenity::{
    framework::standard::Args,
    model::channel::Message,
    prelude::{Context, TypeMapKey},
};
use std::{collections::HashMap, fmt::Write, fs::File, io::Read, path::Path};

pub struct Score;

impl TypeMapKey for Score {
    type Value = HashMap<String, i64>;
}

pub fn init(filename: String) -> HashMap<String, i64> {
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

fn update_score(ctx: &mut Context, msg: &Message, args: &mut Args, update: impl Fn(i64) -> i64) {
    if args.len() != 1 {
        let _ = msg.reply("Donne moi **un** nom !");
        let _ = msg.react('❎');
        return;
    }
    let user: String = args.iter().next().unwrap().unwrap();
    let user = match get_user_id(user) {
        Err(e) => {
            let _ = msg.reply(&e);
            let _ = msg.react('❎');
            return;
        }
        Ok(u) => u,
    };
    let mut data = ctx.data.lock();
    let score = data
        .get_mut::<Score>()
        .expect("Expected Score in ShareMap.");
    let entry = score.entry(user.to_string()).or_insert(0);
    *entry = update(*entry);

    let _ = msg.react('👌');
}

command!(mdr(ctx, msg, args) {
    update_score(ctx, msg, &mut args, |n| n + 1);
    return Ok(());
});

command!(nul(ctx, msg, args) {
    update_score(ctx, msg, &mut args, |n| n - 1);
    return Ok(());
});
