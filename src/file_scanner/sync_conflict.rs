use tokio::{spawn, time::sleep};

use crate::bot::{self, context::DisCtx, destination::Destination};
use std::{collections::HashSet, path::PathBuf, time::Duration};

pub const FUZ_VAULT_PATH: &str = "/home/fuzzy/obsidian/fuz-vault";
pub const MAGIC_BEANS_VAULT_PATH: &str = "/home/fuzzy/obsidian/magic-beans-vault";

pub struct SyncConflictConfig {
    root_dir: &'static str,
    notification_channel: Destination,
}

impl SyncConflictConfig {
    pub fn gen_all_configs() -> Vec<SyncConflictConfig> {
        vec![
            SyncConflictConfig {
                root_dir: FUZ_VAULT_PATH,
                notification_channel: Destination::DebugObsidianCh,
            },
            SyncConflictConfig {
                root_dir: MAGIC_BEANS_VAULT_PATH,
                notification_channel: Destination::MagicBeansObsidianCh,
            },
        ]
    }
}

pub async fn look_for_sync_conflicts(ctx: DisCtx, config: SyncConflictConfig) {
    let mut file_sync_errors = HashSet::new();

    loop {
        let mut path_queue: Vec<PathBuf> = vec![config.root_dir.into()];

        while let Some(path) = path_queue.pop() {
            let paths = std::fs::read_dir(path).unwrap();
            for item in paths.map(Result::unwrap) {
                let path = item.path();

                if path.is_dir() {
                    path_queue.push(path.clone());
                }

                let file_str = match std::str::from_utf8(item.file_name().as_encoded_bytes()) {
                    Ok(s) => s.to_owned(),
                    Err(e) => {
                        let err_msg = format!(
                            "Error at {file}:{line}  with message:```{msg:?}```",
                            file = file!(),
                            line = line!(),
                            msg = e
                        );

                        spawn(bot::report_rust_error(ctx.clone(), err_msg));
                        continue;
                    }
                };

                if file_str.contains("sync-conflict") && file_sync_errors.insert(path.clone()) {
                    let file_name = path
                        .strip_prefix(config.root_dir)
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_owned();

                    spawn(bot::report_file_sync_conflict(
                        ctx.clone(),
                        file_name,
                        path.to_str().unwrap().to_owned(),
                        config.notification_channel,
                    ));
                }
            }
        }
        file_sync_errors.retain(|file| file.exists());

        sleep(Duration::from_secs(5)).await;
    }
}
