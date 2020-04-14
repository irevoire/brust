use rand::seq::SliceRandom;
use rand::thread_rng;
use serenity::framework::standard::{macros::command, Args, CommandResult};

use serenity::{
    model::channel::Message,
    prelude::{Context, TypeMapKey},
};

pub struct Tg;

// we are going to store the insults in the first vector and random index in the second
impl TypeMapKey for Tg {
    type Value = Vec<&'static str>;
}

#[command]
pub fn tg(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
    let mut data = ctx.data.write();

    let insults = data.get_mut::<Tg>();
    if insults.is_none() {
        let _ = msg.reply(&ctx, "Jesus is dead");
        return Ok(());
    }
    let insults = insults.unwrap();
    let mut insult = insults.pop();
    if insult.is_none() {
        *insults = init_tg();
        insult = insults.pop();
    }
    let insult = insult.expect("The insults vector look empty?");

    let _ = msg.channel_id.say(&ctx, insult);
    Ok(())
}

pub fn init_tg() -> Vec<&'static str> {
    let mut insults = vec![
        "Va marcher sur des Légos",
        "Gredin",
        "Tête de tétard",
        "Sac à puces",
        "Espèce d'épinard",
        "Patate",
        "Banane",
        "Capitaine de bateau-lavoir",
        "Cornichon",
        "Paltoquet",
        "Philistin",
        "Terrine",
        "Foutriquet",
        "Scélérat",
        "Mauviette",
        "Malotru",
        "Goujat",
        "Vil faquin",
        "Maraud",
        "Crétin des Alpes",
        "Parisien",
        "Pute",
        "Grosse pute",
        "Petite pute",
        "Trou duc",
        "Chauve",
        "Syndicaliste",
        "Gilet jaune",
        "Macroniste",
        "Fécalomes",
        "Raclure de bidet",
        "Balai à chiotte",
        "Fils de yack",
        "Filloniste",
        "Lepéniste",
        "Mélenchoniste",
        "Sarkozyste",
        "Capitaliste",
        "Homéopathe",
        "Antivax",
        "Moule à gaufre",
        "Escogriffe",
        "Andouille",
        "Tocard",
        "Bouffon",
        "Zigoto",
        "Saltimbanque",
        "Termaji",
        "Branquignole",
        "Enclume",
        "Va te faire cuire le cul",
        "Takezen",
        "Pimfle",
        "Caillou",
        "Goulamas",
        "Gougnafier",
        "Pouffre",
        "Poulpe",
        "Figure de pain sucé",
        "Figure d'angoisse",
        "Front d'endive",
        "Tronc de figuier",
        "Guit",
        "Féministe",
        "Sac à pus",
        "Blaireau",
        "Bordel à Cul",
        "Boudin",
        "Bouffon",
        "Bougre d’âne",
        "Bougre d’imbécile",
        "Bougre de congre",
        "Bougre de conne",
        "Boule de pus",
        "Boursemolle",
        "Branleur",
        "Branlotin",
        "Branque",
        "Branquignole",
        "Brêle",
        "Cagole",
        "Carburateur à Beaujolais",
        "Chiure de pigeon",
        "Clampin",
        "Cloaque",
        "Clodo",
        "Cornegidouille",
        "Couille de tétard",
        "Couille molle",
        "Crétin des îles",
        "Crétin goîtreux",
        "Cul de babouin",
        "Enculeur de mouches",
        "Erreur de la nature",
        "Fils de cheminot",
        "Fils de syndicaliste",
        "Face de cul",
        "Face de pet",
        "Face de rat",
        "Fiente",
        "Fiote",
        "Flaque de pus",
        "Fonctionnaire",
        "Four à merde",
        "Furoncle",
        "Garage à bite",
        "Glandus",
        "Gourdasse",
        "Gourgandine",
        "Grand cornichon",
        "Gras du bide",
        "Grognasse",
        "Gros caca poilu",
        "Grosse truie violette",
        "Gueule de fion",
        "Lèche-cul",
        "Manche à couille",
        "Mange merde",
        "Margoulin",
        "Merdaillon",
        "Merde molle",
        "Moudlabite",
        "Peau de bite",
        "Pecore",
        "Pignouf",
        "Pimbêche",
        "Pisse-vinaigre",
        "Pompe à merde",
        "Pouffe",
        "Pouffiasse",
        "Pute au rabais",
        "Pute borgne",
        "Ramassis de chiure de moineau",
        "Sac à Vin",
        "Sac à Foutre",
        "Vieux",
        "Sauvage",
        "Serpillière à foutre",
        "Tas de saindoux",
        "Thon",
        "Tire couilles",
        "Tête d’ampoule",
        "Tête de bite",
        "Tête de chibre",
        "Tête de con",
        "Tête de noeud",
        "Tête à claques",
        "Vioque",
        "Wisigoth",
        "T’es con comme du plastique",
        "T’es comme une pizza, sauf qu’elle on peut l’avoir sans champignons",
        "Ton père le chauve",
        "Ta mère elle boit l’eau des pâtes",
        "Va pisser sur un fil électrique",
        "Va jouer sur l’autoroute",
        "Va jouer sur la voie ferrée",
        "Beurre doux",
        "Rentre dans ton pays",
        "Avortement raté",
        "Va jouer dans le mixer",
        "Tes parents ils ont jeté le bébé et élevé le placenta",
        "Ta mère en sarouel",
        "T’es moche même de dos",
        "Puterelle",
    ];
    let mut rng = thread_rng();
    insults.shuffle(&mut rng);
    insults
}
