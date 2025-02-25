use crate::{config::destination::Destination, Ctx};
use poise::serenity_prelude as serenity;
use serenity::builder::{CreateAttachment, CreateEmbed};
use std::{convert::Into, sync::Arc};

pub async fn send_msg(
    ctx: crate::Ctx,
    msg: impl Into<String>,
    files: Option<Vec<CreateAttachment>>,
    embeds: Option<Vec<CreateEmbed>>,
    dest: &Destination,
) -> Result<(), Box<dyn std::error::Error>> {
    let id = dest.id(ctx.clone());

    let builder = serenity::builder::CreateMessage::new().content(msg);

    let builder = match embeds.clone() {
        Some(embeds) => builder.embeds(embeds),
        None => builder,
    };

    let builder = match files.clone() {
        Some(files) => builder.files(files),
        None => builder,
    };
    id.await.send_message(ctx.dis_ctx.clone(), builder).await?;

    Ok(())
}

pub async fn report_reminder_notification(
    ctx: crate::Ctx,
    reminder_msg: String,
    dest: String,
    time: String,
    file: String,
) {
    println!("report_reminder_notification {reminder_msg}  time: {time}");

    let embed = CreateEmbed::new()
        .title(reminder_msg.clone())
        .description(format!("time: {time}\nfile: {file}"));

    send_msg(ctx.clone(), reminder_msg, None, Some(vec![embed]), ctx.get_dest(&dest))
        .await
        .unwrap();
}

pub async fn report_file_sync_conflict(
    ctx: crate::Ctx,
    file_name: String,
    file: String,
    vault_name: Arc<String>
) {
    let err_msg = format!("file sync conflict: {file_name}");
    println!("report_file_sync_conflict: {err_msg}");
    let file = CreateAttachment::path(file).await.unwrap();
    send_msg(ctx.clone(), err_msg, Some(vec![file]), None, ctx.get_vault_dest(&*vault_name))
        .await
        .unwrap();
}

pub async fn report_rust_error(ctx: Ctx, err_msg: String) {
    eprintln!("err msg: {}", err_msg);
    send_msg(ctx.clone(), err_msg, None, None, ctx.get_error_ch())
        .await
        .unwrap();
}
