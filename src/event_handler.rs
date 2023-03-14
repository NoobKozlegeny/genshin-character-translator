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

use self::extensions::*;

#[path = "extensions.rs"] mod extensions;
#[path = "character.rs"] mod character;

use crate::event_handler::character::Character;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {

        // authors are the users we will interact with
        let authors: Vec<&str> = Vec::from([ "NoobKözlegény", "Husania" ]);

        let genshin_names: Vec<Character> = Vec::from([
            Character::new(&["ALBERT"], "ALBEDO"), Character::new(&["ALIHAN"], "ALHAITHAM"),
            Character::new(&["BORI"], "AMBER"), Character::new(&["ÁNGYIKA"], "AYAKA"),
            Character::new(&["AJTONY"], "AYATO"), Character::new(&["ISTVÁN"], "ITTO"),
            Character::new(&["BARBI"], "BARBARA"), Character::new(&["BERNADETT"], "BEIDOU"),
            Character::new(&["BÉNI"], "BENNETT"), Character::new(&["KASSZANDRA"], "CANDACE"),
            Character::new(&["CSONGI"], "CHONGYUN"), Character::new(&["CSANÁD"], "CHILDE"),
            Character::new(&["KOLLI"], "COLLEI"), Character::new(&["SANYÓ"], "CYNO"),
            Character::new(&["DÉNES"], "DAINSLEIF"), Character::new(&["DANI"], "DILUC"),
            Character::new(&["DIA"], "DIONA"), Character::new(&["DÓRI"], "DORI"),
            Character::new(&["LAURA"], "EULA"), Character::new(&["FRUZSI", "FRUZSINA"], "FARUZAN"),
            Character::new(&["FLÓRA"], "FISCHL"), Character::new(&["GYÖNGYI"], "GANYU"),
            Character::new(&["GERGŐ"], "GOROU"), Character::new(&["HETÉNY"], "HEIZHOU"),
            Character::new(&["TEA"], "HUTAO"), Character::new(&["JANKA"], "JEAN"),
            Character::new(&["KÁZMÉR"], "KAZUHA"), Character::new(&["KÁROLY"], "KAEYA"),
            Character::new(&["KORINA"], "KOKOMI"), Character::new(&["SÜTI"], "KUKI"),
            Character::new(&["KEKA", "KINGA"], "KEQING"), Character::new(&["KLÁRI"], "KLEE"),
            Character::new(&["LIZA"], "LISA"), Character::new(&["MÓNI", "MÓNIKA"], "MONA"),
            Character::new(&["NATI"], "NAHIDA"), Character::new(&["NIKI"], "NINGUANQ"),
            Character::new(&["NÍLA"], "NILOU"), Character::new(&["NOELLA"], "NOELLE"),
            Character::new(&["GIZI"], "QIGI"), Character::new(&["ÉVI"], "RAIDEN"),
            Character::new(&["RADIÁTOR"], "RAZOR"), Character::new(&["ROZI"], "ROSARIA"),
            Character::new(&["SÁRA"], "SARA"), Character::new(&["SZANDI"], "SAYU"),
            Character::new(&["SZENDE"], "SHENHE"), Character::new(&["CUKOR"], "SUCROSE"),
            Character::new(&["TOMI"], "THOMA"), Character::new(&["TIHAMÉR"], "TIGHNARI"),
            Character::new(&["UTAZÓ"], "TRAVELER"), Character::new(&["BARNI"], "VENTI"),
            Character::new(&["SAMU"], "XIAO"), Character::new(&["SZINDBÁD"], "XINGQIU"),
            Character::new(&["SZINTIA"], "XINYAN"), Character::new(&["CSINGILING"], "XIANGLINQ"),
            Character::new(&["JANA"], "YANFEI"), Character::new(&["MOLLI"], "YAEMIKO"),
            Character::new(&["JOLÁN"], "YELAN"), Character::new(&["JOLI"], "YOIMIJA"),
            Character::new(&["JULI"], "YUN JIN"), Character::new(&["SALAMON"], "SCARA"),
            Character::new(&["ZOLI"], "ZHONGLI")
        ]);

        let hu_name: String = select_name(msg.content.clone(), genshin_names.clone());

        if authors.contains(&&msg.author.name[..]) 
        && genshin_names.contains_character(hu_name.clone()) {
            react_to_message(ctx, msg, genshin_names, hu_name).await;            
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

/// Selects the Hungarian genshin name from the string
/// # Parameters
///     input: the string to search in
///     genshin_names: HashMap with the corresponding names
fn select_name(input: String, genshin_names: Vec<Character>) -> String {
    for item in input.to_uppercase().split(" ") {
        for character in genshin_names.iter() {
            for hu_name in character.clone().hu_names {
                if item.to_string().starts_with(&hu_name) {
                    return hu_name.to_owned();
                }   
            }
        }
    }

    return "".to_owned();
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
async fn react_to_message(ctx: Context, msg: Message, genshin_names: Vec<Character>, hu_name: String) {
    // Getting the correct english equivalent name
    let en_name: String = genshin_names.get_en_name(hu_name).unwrap();

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