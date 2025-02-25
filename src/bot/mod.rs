use self::context::CacheAndHttp;
use crate::config::Context;
use poise::serenity_prelude::all::{Client, GatewayIntents};
use std::{sync::Arc,time::Duration};

pub mod commands;
pub mod context;
pub mod send_msgs;

type Error = Box<dyn std::error::Error + Send + Sync>;
type PoiseContext<'a> = poise::Context<'a, BotData, Error>;

pub struct BotData {}

async fn on_error(error: poise::FrameworkError<'_, BotData, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

pub async fn bot_start(config: crate::config::Config) -> (crate::config::Context, Client) {
    let options = poise::FrameworkOptions {
        commands: vec![commands::get_reminders()],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("~".into()),
            edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                Duration::from_secs(3600),
            ))),
            ..Default::default()
        },
        // The global error handler for all error cases that may occur
        on_error: |error| Box::pin(on_error(error)),
        // This code is run before every command
        pre_command: |ctx| {
            Box::pin(async move {
                println!("Executing command {}...", ctx.command().qualified_name);
            })
        },
        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(async move {
                println!(
                    "Got an event in event handler: {:?}",
                    event.snake_case_name()
                );
                Ok(())
            })
        },
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(BotData {})
            })
        })
        .options(options)
        .build();

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
