use std::sync::Arc;


use self::context::{CacheAndHttp, DisCtx};
use self::destination::Destination;
use serenity::all::{Client, GatewayIntents};
use serenity::builder::{CreateAttachment, CreateMessage};
use serenity::framework::standard::{Configuration, StandardFramework};



pub mod context;
pub mod destination;

pub async fn bot_start() -> (DisCtx, Client){
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


pub async fn send_msg(
    ctx: DisCtx,
    msg: impl Into<String>,
    files: Option<Vec<CreateAttachment>>,
    dest: Destination,
) -> Result<(), Box<dyn std::error::Error>> {
    let id = dest.id(ctx.clone());

    match files.clone() {
        Some(files) if !files.is_empty() => {
            id.await
                .send_files(
                    ctx,
                    files,
                    CreateMessage::new().content(msg),
                )
                .await?
        }
        _ => {
            id.await
                .say(ctx, msg.into())
                .await?
        }
    };

    Ok(())
}

pub async fn report_file_sync_conflict(ctx: DisCtx, file_name: String, file: String, dest: Destination) {
    let err_msg = format!("file sync conflict: {file_name}");
    println!("{err_msg}");
    let file = CreateAttachment::path(file).await.unwrap();
    send_msg(ctx, err_msg, Some(vec![file]), dest).await.unwrap();
}

pub async fn report_rust_error(ctx: DisCtx, err_msg: String) {
    eprintln!("err msg: {}", err_msg);
    send_msg(ctx, err_msg, None, Destination::DebugObsidianCh).await.unwrap();
}
