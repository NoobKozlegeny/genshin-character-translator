// Imports
use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::*;
use serenity::model::prelude::{Ready};
use std::collections::HashMap;
use std::vec::Vec;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufReader};
use std::io::prelude::*;
use std::path::Path;
use std::str;

use self::extensions::VecExt;

#[path = "extensions.rs"] mod extensions;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {

        // authors are the users we will interact with
        let authors: Vec<&str> = Vec::from([ "NoobKözlegény" ]);
        let genshin_names: HashMap<String, String> = HashMap::from([
            (String::from("ALBERT"), String::from("ALBEDO")), (String::from("ALI"), String::from("ALHAITHAM")),
            (String::from("BORI"), String::from("AMBER")), (String::from("ÁNGYIKA"), String::from("AYAKA")),
            (String::from("AJTONY"), String::from("AYATO")), (String::from("ISTVÁN"), String::from("ITTO")),
            (String::from("BARBI"), String::from("BARBARA")), (String::from("BERNADETT"), String::from("BEIDOU")),
            (String::from("BÉNI"), String::from("BENNETT")), (String::from("KASSZANDRA"), String::from("CANDACE")),
            (String::from("CSONGI"), String::from("CHONGYUN")), (String::from("CSANÁD"), String::from("CHILDE")),
            (String::from("KOLLI"), String::from("COLLEI")), (String::from("SANYÓ"), String::from("CYNO")),
            (String::from("DÉNES"), String::from("DAINSLEIF")), (String::from("DANI"), String::from("DILUC")),
            (String::from("DIA"), String::from("DIONA")), (String::from("DÓRI"), String::from("DORI")),
            (String::from("LAURA"), String::from("EULA")), (String::from("FRUZSI"), String::from("FRUZSINA")),
            (String::from("FLÓRA"), String::from("FISCHL")), (String::from("GYÖNGYI"), String::from("GANYU")),
            (String::from("GERGŐ"), String::from("GOROU")), (String::from("HETÉNY"), String::from("HEIZHOU")),
            (String::from("TEA"), String::from("HU TAO")), (String::from("JANKA"), String::from("JEAN")),
            (String::from("KÁZMÉR"), String::from("KAZUHA")), (String::from("KÁROLY"), String::from("KAEYA")),
            (String::from("KORINA"), String::from("KOKOMI")), (String::from("SÜTI"), String::from("KUKI")),
            (String::from("KEKA"), String::from("KEQING")), (String::from("KINGA"), String::from("KEQING")),
            (String::from("KLÁRI"), String::from("KLEE")), (String::from("LEJLA"), String::from("LAYLA")),
            (String::from("LIZA"), String::from("LISA")), (String::from("MÓNI"), String::from("MONA")),
            (String::from("NATI"), String::from("NAHIDA")), (String::from("NIKI"), String::from("NINGUANQ")),
            (String::from("NÍLA"), String::from("NILOU")), (String::from("NOELLA"), String::from("NOELLE")),
            (String::from("GIZI"), String::from("QIGI")), (String::from("ÉVI"), String::from("RAIDEN")),
            (String::from("RADIÁTOR"), String::from("RAZOR")), (String::from("ROZI"), String::from("ROSARIA")),
            (String::from("SÁRA"), String::from("SARA")), (String::from("SZANDI"), String::from("SAYU")),
            (String::from("SZENDE"), String::from("SHENHE")), (String::from("CUKOR"), String::from("SUCROSE")),
            (String::from("TOMI"), String::from("THOMA")), (String::from("TIHAMÉR"), String::from("TIGHNARI")),
            (String::from("UTAZÓ"), String::from("TRAVELER")), (String::from("BARNI"), String::from("VENTI")),
            (String::from("SAMU"), String::from("XIAO")), (String::from("SZINDBÁD"), String::from("XINGQIU")),
            (String::from("SZINTIA"), String::from("XINYAN")), (String::from("CSINGILING"), String::from("XIANGLINQ")),
            (String::from("JANA"), String::from("YANFEI")), (String::from("MOLLI"), String::from("YAE MIKO")),
            (String::from("JOLÁN"), String::from("YELAN")), (String::from("JOLI"), String::from("YOIMIJA")),
            (String::from("JULI"), String::from("YUN JIN")), (String::from("SALAMON"), String::from("SCARA")),
            (String::from("ZOLI"), String::from("ZHONGLI"))
        ]);

        if authors.contains(&&msg.author.name[..]) 
        && genshin_names.contains_key(&msg.content.to_uppercase()) {
            react_to_message(ctx, msg, genshin_names).await;            
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

/// Reads the genshin names into a HashMap<String, String>
/// # Parameters
///     path: a &Path struct pointing to the file location
fn read_names(path: &Path) -> HashMap<String, String> {
    let mut result: HashMap<String, String> = HashMap::new();
    let file = File::open(path).expect("File not found!");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_ok = line.unwrap().to_string().to_uppercase(); // Zhongli - Zoli
        let names: Vec<&str> = line_ok.split("-").collect();

        // Bc we 'translate' the hungarian to the english one, hence the idx switchup
        result.insert(names[1].trim().to_string().clone(), names[0].trim().to_string().clone()); 
    }

    return result;
}

/// Spells out the given genshin character's name/nickname with emojis.
/// # Parameters
///     ctx: Context file of the message
///     msg: Message object
///     genshin_names: A HashMap<&str, &str> with genshin names (<name to react, word to spell>) 
async fn react_to_message(ctx: Context, msg: Message, genshin_names: HashMap<String, String>) {
    // Getting the correct english equivalent name
    let msg_content = msg.content.to_uppercase().clone();
    let en_name: String = get_correct_value(msg_content, genshin_names.clone()).unwrap();

    // Defining the letters with their corresponding emojis
    let emoji_utf_primary: HashMap<&str, &str> = HashMap::from([
        ("A", "🇦"), ("B", "🇧"), ("C", "🇨"), ("D", "🇩"),
        ("E", "🇪"), ("F", "🇫"), ("G", "🇬"), ("H", "🇭"),
        ("I", "🇮"), ("J", "🇯"), ("K", "🇰"), ("L", "🇱"),
        ("M", "🇲"), ("N", "🇳"), ("O", "🇴"), ("P", "🇵"),
        ("Q", "🇶"), ("R", "🇷"), ("S", "🇸"), ("T", "🇹"),
        ("U", "🇺"), ("V", "🇻"), ("W", "🇼"), ("X", "🇽"),
        ("Y", "🇾"), ("Z", "🇿")
    ]);
    let emoji_utf_secondary: HashMap<&str, &str> = HashMap::from([
        ("A", "🅰️"), ("B", "🅱️"), ("C", "☪️"), ("I", "ℹ️"),
        ("M", "Ⓜ️"), ("O", "🅾️"), ("P", "🅿️"), ("U", "⛎"),
        ("T", "✝️"), ("X", "❌"), ("V", "🔽") 
    ]);

    // Turns the name into characters and removes the accents to help display later
    let en_name_letters: Vec<char> = en_name.chars().collect();
    en_name_letters.replace_accents();

    // Iterating through all the character letters
    for (i, item) in en_name_letters.iter().enumerate() {
        let emoji_utf: String;

        // If we wish to use an emoji which is already present on the message, then we
        // search for an alternative in emoji_utf_secondary
        if (i == 0) || !en_name_letters[0..i].contains(&item) {
            emoji_utf = get_correct_value(&item.to_string()[..], emoji_utf_primary.clone()).unwrap().to_owned();
        }
        else {
            emoji_utf = get_correct_value(&item.to_string()[..], emoji_utf_secondary.clone()).unwrap_or_else(|| {
                "⚠️"
            }).to_owned();
        }

        // This creates a "⚠️" emoji to display it couldn't find an emoji for the associated letter
        let emoji: ReactionType = ReactionType::try_from(emoji_utf).unwrap_or_else(|_error: ReactionConversionError| {
            return ReactionType::try_from("⚠️").unwrap();
        });

        if !emoji.unicode_eq("⚠️") {
            if let Err(why) = msg.react(&ctx, emoji).await {
                println!("{why}");
            }
        }
    }
}

/// Returns the corresponding value to the input key if present
/// # Parameters
///     input: A T variable. (This is the key)
///     hash_map: A <T, T> hashmap duuuh
fn get_correct_value<T: Hash + Eq + Clone>(input: T, hash_map: HashMap<T, T>) -> Option<T> {
    match hash_map.get(&input) {
        Some(x) => return Some(x.clone()),
        None => println!("Key doesn't exist in hash_map")
    }

    return None;
}