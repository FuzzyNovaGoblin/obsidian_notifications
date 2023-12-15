use super::context::DisCtx;
use super::destination::Destination;
use serenity::builder::{CreateAttachment, CreateEmbed};

pub async fn send_msg(
    ctx: DisCtx,
    msg: impl Into<String>,
    files: Option<Vec<CreateAttachment>>,
    embeds: Option<Vec<CreateEmbed>>,
    dest: Destination,
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

    id.await.send_message(ctx, builder).await?;

    Ok(())
}

pub async fn report_reminder_notification(
    ctx: DisCtx,
    reminder_msg: String,
    dest: Destination,
    time: String,
    file: String
) {
    println!("report_reminder_notification {reminder_msg}  time: {time}");

    let embed = CreateEmbed::new().title(reminder_msg.clone()).description(format!("time: {time}\nfile: {file}"));

    send_msg(ctx, reminder_msg, None, Some(vec![embed]), dest)
        .await
        .unwrap();
}

pub async fn report_file_sync_conflict(
    ctx: DisCtx,
    file_name: String,
    file: String,
    dest: Destination,
) {
    let err_msg = format!("file sync conflict: {file_name}");
    println!("report_file_sync_conflict: {err_msg}");
    let file = CreateAttachment::path(file).await.unwrap();
    send_msg(ctx, err_msg, Some(vec![file]), None, dest)
        .await
        .unwrap();
}

pub async fn report_rust_error(ctx: DisCtx, err_msg: String) {
    eprintln!("err msg: {}", err_msg);
    send_msg(ctx, err_msg, None, None, Destination::FuzzyObsidianCh)
        .await
        .unwrap();
}
