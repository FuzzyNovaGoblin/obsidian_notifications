use self::context::CacheAndHttp;
use crate::config::Context;
use serenity::all::{Client, GatewayIntents};
use serenity::framework::standard::{Configuration, StandardFramework};
use std::sync::Arc;

pub mod context;
pub mod send_msgs;

pub async fn bot_start(config: crate::config::Config) -> (crate::config::Context, Client) {
    let framework = StandardFramework::new();
    framework.configure(Configuration::new().prefix("~"));

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(config.bot_token.clone(), intents)
        .framework(framework)
        .await
        .expect("Error creating client");
    let dis_ctx = CacheAndHttp::new(client.http.clone(), client.cache.clone());

    (
        Context {
            dis_ctx: Arc::new(dis_ctx),
            config,
        },
        client,
    )
}
