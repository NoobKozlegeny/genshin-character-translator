// Imports
use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::*;
use serenity::model::prelude::{Ready};
use std::collections::*;
use std::str;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {

        // authors is the users we will interact with
        let authors: Vec<&str> = Vec::from([ "NoobKözlegény" ]); 
        let genshin_names: HashMap<&str, &str> = HashMap::from([
            ("Albedo", "Albert"), ("Alhaitham", "Ali"), ("Aloy", ""),
            ("Amber", "Bori"), ("Ayaka", "Abacc"), ("Ayato", "Ajtony")
        ]);

        if authors.contains(&&msg.author.name[..]) 
        && genshin_names.contains_key(&&msg.content[..]) {
            react_to_message(ctx, msg, genshin_names).await;            
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

/// Spells out the given genshin character's name/nickname with emojis.
/// # Parameters
///     ctx: Context file of the message
///     msg: Message object
///     genshin_names: A HashMap<&str, &str> with genshin names (<name to react, word to spell>) 
async fn react_to_message(ctx: Context, msg: Message, genshin_names: HashMap<&str, &str>) {
    // Getting the correct english equivalent name
    let en_name: String = get_correct_value(&&msg.content[..], genshin_names.clone());

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
        ("A", "🅰️"), ("B", "🅱️"), ("M", "Ⓜ️"), ("O", "🅾️")
    ]);

    let en_name_letters: Vec<char> = en_name.to_uppercase().chars().collect();

    // Iterating through all the character letters
    for (i, item) in en_name_letters.iter().enumerate() {
        let emoji_utf: String;

        // If we wish to use an emoji which is already present on the message, then we
        // search for an alternative in emoji_utf_secondary
        if (i == 0) || !en_name_letters[0..i].contains(&item) {
            emoji_utf = get_correct_value(&item.to_string()[..], emoji_utf_primary.clone());            
        }
        else {
            emoji_utf = get_correct_value(&item.to_string()[..], emoji_utf_secondary.clone());
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
///     input: A &str variable. (This is the key)
///     hash_map: A <&str, &str> hashmap duuuh
/// 
/// TODO: Make this generic pls
fn get_correct_value(input: &str, hash_map: HashMap<&str, &str>) -> String {
    let mut result: String = String::from("");

    match hash_map.get(&input) {
        Some(&value) => result = value.to_string(),
        _ => println!("Couldn't find key in emoji_letters_XYZ")
    };

    return result;
}