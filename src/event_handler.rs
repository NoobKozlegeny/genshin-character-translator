// Imports
use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::*;
use serenity::model::prelude::{Ready};
use std::borrow::Borrow;
use std::collections::*;
use std::fmt::Display;
use std::fs::File;
use std::hash::Hash;
use std::io::Error;
use std::io::{BufReader, Result};
use std::io::prelude::*;
use std::path::Path;
use std::str;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {

        // authors is the users we will interact with
        let authors: Vec<&str> = Vec::from([ "NoobKözlegény" ]);
        let genshin_names: HashMap<String, String> = read_names(Path::new("names.txt"));

        if authors.contains(&&msg.author.name[..]) 
        && genshin_names.contains_key(&msg.content) {
            react_to_message(ctx, msg, genshin_names).await;            
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn read_names(path: &Path) -> HashMap<String, String> {
    let mut result: HashMap<String, String> = HashMap::new();
    let file = File::open(path).expect("File not found!");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_ok = line.unwrap().to_string(); // Zhongli - Zoli
        let names: Vec<&str> = line_ok.split("-").collect();
        
        // Bc we 'translate' the hungarian to the english one, hence the idx switchup
        result.insert(names[1].trim().to_owned(), names[0].trim().to_owned()); 
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
    let msg_content = msg.content.clone();
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
        ("A", "🅰️"), ("B", "🅱️"), ("M", "Ⓜ️"), ("O", "🅾️"), ("I", "ℹ️")
    ]);

    let en_name_letters: Vec<char> = en_name.to_uppercase().chars().collect();

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
/// 
/// TODO: Make a better actual generic pls
fn get_correct_value<T: Hash + Eq + Clone>(input: T, hash_map: HashMap<T, T>) -> Option<T> {
    // let mut result: Option<T> = None;

    // match hash_map.get(&input) {
    //     Some(&value) => result = value.into(),
    //     _ => println!("Couldn't find key in emoji_letters_XYZ")
    // };

    // let value = hash_map.get(&input).unwrap().clone();

    match hash_map.get(&input) {
        Some(x) => return Some(x.clone()),
        None => println!("Key doesn't exist in hash_map")
    }

    return None;
}