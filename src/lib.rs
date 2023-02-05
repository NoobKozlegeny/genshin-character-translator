use anyhow::anyhow;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
mod event_handler;

#[shuttle_service::main]
async fn serenity(#[shuttle_secrets::Secrets] secret_store: SecretStore) 
-> shuttle_service::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT 
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MESSAGE_REACTIONS;

    let client = Client::builder(&token, intents)
        .event_handler(event_handler::Handler)
        .await
        .expect("Err creating client");

    Ok(client)
}