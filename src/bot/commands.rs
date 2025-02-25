use super::{Error, PoiseContext};
use crate::{config::Config, state::State};
use poise::{serenity_prelude::CreateAttachment, CreateReply};
use shared_singleton::Singleton;

#[poise::command(slash_command)]
pub async fn get_reminders<'a>(ctx: PoiseContext<'a>) -> Result<(), Error> {
    let cfg = Config::singleton();
    let sys_state = State::singleton();

    let mut dest = None;
    for d in &cfg.destinations {
        if ctx.channel_id() == d.1.id {
            dest = Some(d);
            break;
        }
    }

    let dest = match dest {
        None => {
            ctx.say("Get the fuck out").await?;
            return Ok(());
        }
        Some(v) => v.0,
    };

    let mut vault_name = None;
    for (name, vault) in &cfg.vaults {
        if &vault.destination == dest {
            vault_name = Some(name);
            break;
        }
    }

    let vault_name = match vault_name {
        None => {
            ctx.say("Get the fuck out").await?;
            return Ok(());
        }
        Some(v) => v,
    };

    let mut ret_strings = Vec::<String>::new();

    let lock = sys_state.lock().await;
    let reminders = match lock.reminders.get(vault_name) {
        Some(v) => v,
        None => {
            ctx.say("reminders contained None, check back later")
                .await?;
            return Ok(());
        }
    };

    for reminder in reminders {
        let reminder_string = format!(
            "- time: {time}\n  location: {loc}\n  msg: `{msg}`",
            time = reminder.time_parts.discord_display(),
            loc = reminder
                .file_path
                .strip_prefix(&cfg.vaults.get(vault_name).unwrap().root_dir)
                .unwrap_or(&reminder.file_path)
                .trim_matches('/'),
            msg = reminder.msg
        );

        ret_strings.push(reminder_string);
    }

    let msg = ret_strings.join("\n");
    let mut reply = CreateReply::default().attachment(CreateAttachment::bytes(
        format!("{:#?}", reminders),
        "RawReminders.txt",
    ));
    reply = reply.attachment(CreateAttachment::bytes(
        msg.bytes().collect::<Vec<_>>(),
        "Reminders.md",
    ));
    if msg.len() > 2000 {
        reply = reply.attachment(CreateAttachment::bytes(
            msg.bytes().collect::<Vec<_>>(),
            "Reminders.md",
        ));
    } else {
        reply = reply.content(msg);
    };
    ctx.send(reply).await?;
    Ok(())
}
