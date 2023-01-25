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
            ("Joli", "Yoi"),
            ("Kolli", "Colei")
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
    let mut en_name: String = String::from("");

    match genshin_names.get(&&msg.content[..]) {
        Some(&value) => en_name.push_str(value),
        _ => println!("Couldn't find key in genshin_names")
    };

    // Reacting with the correct emojis
    let emoji_letters: HashMap<&str, &str> = HashMap::from([
        ("A", "🇦"), ("B", "🇧"), ("C", "🇨"), ("D", "🇩"),
        ("E", "🇪"), ("F", "🇫"), ("G", "🇬"), ("H", "🇭"),
        ("I", "🇮"), ("J", "🇯"), ("K", "🇰"), ("L", "🇱"),
        ("M", "🇲"), ("N", "🇳"), ("O", "🇴"), ("P", "🇵"),
        ("Q", "🇶"), ("R", "🇷"), ("S", "🇸"), ("T", "🇹"),
        ("U", "🇺"), ("V", "🇻"), ("W", "🇼"), ("X", "🇽"),
        ("Y", "🇾"), ("Z", "🇿")
    ]);

    let en_name_letters: Vec<char> = en_name.chars().collect();

    for item in en_name_letters {
        let mut letter = String::from("");

        match emoji_letters.get(&item.to_string().to_uppercase()[..]) {
            Some(&value) => letter.push_str(value),
            _ => println!("Couldn't find key in emoji_letters")
        };

        let emoji = ReactionType::try_from(letter).unwrap();

        if let Err(why) = msg.react(&ctx, emoji).await {
            println!("{why}");
        }
    }
}