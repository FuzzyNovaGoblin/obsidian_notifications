use self::context::CacheAndHttp;
use crate::config::Context;
use serenity::all::{
    Client, CreateInteractionResponse, CreateInteractionResponseMessage,
    EventHandler, GatewayIntents, Interaction, Ready,
};
use serenity::async_trait;
use serenity::framework::standard::{Configuration, StandardFramework};
use std::sync::Arc;

pub mod commands;
pub mod context;
pub mod send_msgs;

pub async fn bot_start(config: crate::config::Config) -> (crate::config::Context, Client) {
    let framework = StandardFramework::new();
    framework.configure(Configuration::new().prefix("~"));

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(config.bot_token.clone(), intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Error creating client");
    let dis_ctx = CacheAndHttp::new(client.http.clone(), client.cache.clone());

    // let get_reminders_cmd = CreateCommand::new("get_reminders").fun

    (
        Context {
            dis_ctx: Arc::new(dis_ctx),
            config,
        },
        client,
    )
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(
        &self,
        ctx: serenity::prelude::Context,
        interaction: serenity::all::Interaction,
    ) {
        if let Interaction::Command(command) = interaction {
            let content = match command.data.name.as_str() {
                "get_reminders" => Some(commands::get_reminders::run(command.channel_id).await),
                _ => Some("not implemented :".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }

    async fn ready(&self, ctx: serenity::prelude::Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        for guild in ready.guilds {
            let guild_id = guild.id;

            let _commands = guild_id
                .set_commands(
                    &ctx.http,
                    vec![
                        commands::get_reminders::register(),
                    ],
                )
                .await;
        }
    }
}
