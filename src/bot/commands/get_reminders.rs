use crate::{config::Config, state::State};
use serenity::{all::ChannelId, builder::CreateCommand};
use shared_singleton::Singleton;

pub async fn run(channel_id: ChannelId) -> String {
    let cfg = Config::singleton();
    let sys_state = State::singleton();

    let mut dest = None;
    for d in &cfg.destinations {
        if d.1.id == channel_id.get() {
            dest = Some(d);
            break;
        }
    }

    let dest = match dest {
        None => return "Get the fuck out".to_owned(),
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
        None => return "Get the fuck out".to_owned(),
        Some(v) => v,
    };

    let mut ret_strings = Vec::<String>::new();

    let lock = sys_state.lock().await;
    let reminders = match lock.reminders.get(vault_name) {
        Some(v) => v,
        None => return "reminders contained None, check back later".into(),
    };

    for reminder in reminders {
        let reminder_string = format!(
            "- time: {time}\n  location: {loc}\n  msg: `{msg}`",
            time = reminder.time_parts.discord_display(),
            loc = reminder
                .file_path
                .strip_prefix(&cfg.vaults.get(vault_name).unwrap().root_dir)
                .unwrap_or(&reminder.file_path).trim_matches('/'),
            msg = reminder.msg
        );

        ret_strings.push(reminder_string);

        dbg!(&ret_strings);
    }

    let msg = ret_strings.join("\n");
    if msg.len() > 2000 {
        format!(
            "{m}\n...{} more characters",
            msg.len() - 1968,
            m = &msg[0..1968],
        )
    } else {
        msg
    }

}

pub fn register() -> CreateCommand {
    CreateCommand::new("get_reminders").description("tmp add later")
}
