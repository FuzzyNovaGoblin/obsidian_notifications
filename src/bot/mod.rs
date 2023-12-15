use self::context::{CacheAndHttp, DisCtx};
use serenity::all::{Client, GatewayIntents};
use serenity::framework::standard::{Configuration, StandardFramework};
use std::sync::Arc;

pub mod context;
pub mod destination;
pub mod send_msgs;

pub async fn bot_start() -> (DisCtx, Client) {
    let framework = StandardFramework::new();
    framework.configure(Configuration::new().prefix("~"));

    let token = include!("bot_token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(token, intents)
        .framework(framework)
        .await
        .expect("Error creating client");
    let ctx = CacheAndHttp::new(client.http.clone(), client.cache.clone());

    (Arc::new(ctx), client)
}
