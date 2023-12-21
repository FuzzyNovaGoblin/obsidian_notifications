use self::model::{DateTimeParts, ReminderKey};
use crate::{
    bot::send_msgs::{report_reminder_notification, report_rust_error},
    file_scanner::path_str_no_root,
};
use chrono::prelude::*;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::PathBuf,
    time::Duration, sync::Arc,
};
use tokio::{spawn, task::JoinHandle, time::sleep};

mod model;

const DATE_TIME_REGEX_STR: &str = r#"(?:- (?<checkbox>\[(?<checked>.)])?)\s?(?<message>.*)\(@(?<date>(?<year>\d{2}|\d{4}).(?<month>\d{1,2}).(?<day>\d{1,2}))?(?:[-\s](?<time>(?<hour>\d{1,2}):?(?<minute>\d{2})(?:(?<second>\d{2}))?))?\)"#;
const _OBS_URI_TEMPLATE: &str =
    "obsidian://advanced-uri?vault={vault_name}&filepath={file_path}&line={line_num}";

pub async fn look_for_time_reminders(ctx: crate::Ctx, vault_name: Arc<String>) {
    let vault = &ctx.config.vaults[&*vault_name];
    let mut reminders: HashMap<ReminderKey, Option<JoinHandle<()>>> = HashMap::new();
    let date_time_regex =
        Regex::new(DATE_TIME_REGEX_STR).expect("failed to compile RegEx dateTimeReg");
    let ignore_files =
        Regex::new(r#"(\/\.DS_Store$)|(\.[jJ][pP][gG]$)|(\.[jJ][pP][eE][gG]$)|(\.[pP][nN][gG]$)"#)
            .expect("failed to compile RegEx ignore_files");
    let ignore_paths = Regex::new(r#"(^.trash)"#).expect("failed to compile RegEx ignore_paths");

    loop {
        let mut discovered_pass: HashSet<ReminderKey> = HashSet::new();

        let mut path_queue: Vec<PathBuf> = vec![vault.root_dir.clone().into()];

        while let Some(path) = path_queue.pop() {
            let paths = std::fs::read_dir(path).unwrap();
            for item in paths.map(Result::unwrap) {
                let path = item.path();
                if path.is_dir() {
                    if ignore_paths.is_match(&path_str_no_root(&path, vault.root_dir.clone())) {
                        continue;
                    }
                    path_queue.push(path.clone());
                } else if ignore_files.is_match(path.to_str().unwrap()) {
                    continue;
                } else {
                    let file_data = match fs::read_to_string(&path) {
                        Ok(v) => v,
                        Err(e) => {
                            let err_msg = format!(
                            "Error reading file {path_name}\nError at {file}:{line}  with message:```{msg:?}```",
                            path_name=path.to_str().unwrap(),
                            file = file!(),
                            line = line!(),
                            msg = e
                        );

                            spawn(report_rust_error(ctx.clone(), err_msg));
                            continue;
                        }
                    };

                    for line in file_data.lines() {
                        for cap in date_time_regex.captures_iter(line) {
                            let time_parts = match DateTimeParts::new(&cap) {
                                Ok(v) => v,
                                Err(e_msg) => {
                                    spawn(report_rust_error(ctx.clone(), e_msg));
                                    continue;
                                }
                            };

                            let key = ReminderKey {
                                file_path: path.to_str().unwrap().into(),
                                msg: cap
                                    .name("message")
                                    .map(|msg| msg.as_str().to_owned())
                                    .unwrap_or_default(),
                                time_parts,
                                completed_checked: cap
                                    .name("checked")
                                    .map(|msg| msg.as_str() != " ")
                                    .unwrap_or(false),
                            };

                            discovered_pass.insert(key.clone());

                            if !reminders.contains_key(&key) {
                                reminders.insert(
                                    key.clone(),
                                    Some(spawn(countdown_reminder(
                                        ctx.clone(),
                                        vault_name.to_owned(),
                                        key.clone(),
                                    ))),
                                );
                            }
                        }
                    }
                }
            }
        }

        for key in get_non_existing(&discovered_pass, &reminders) {
            if let Some((_, Some(j_handle))) = reminders.remove_entry(&key) {
                if !j_handle.is_finished() {
                    j_handle.abort();
                }
            }
        }

        for (_k, j) in reminders.iter_mut() {
            if let Some(j_handle) = j {
                if j_handle.is_finished() {
                    *j = None;
                }
            }
        }

        sleep(Duration::from_secs(5)).await;
    }
}

async fn countdown_reminder(ctx: crate::Ctx, vault_name: Arc<String>, key: ReminderKey) {
    if key.completed_checked {
        return;
    }

    let target_time = key.time_parts.get_target_time();

    let target_time = match target_time.single() {
        Some(v) => v,
        None => {
            let err_msg = format!(
                "Error time was ambiguous time{:?}\nError at {file}:{line}",
                target_time,
                file = file!(),
                line = line!(),
            );

            spawn(report_rust_error(ctx, err_msg));
            panic!()
        }
    };

    let dif = target_time.signed_duration_since(Local::now());

    if dif.abs() == dif {
        sleep(dif.to_std().unwrap()).await;
    } else if dif.abs().to_std().unwrap() >= Duration::from_secs(60) {
        return;
    }

    spawn(report_reminder_notification(
        ctx.clone(),
        key.msg,
        ctx.config.vaults[vault_name.as_ref()].destination.clone(),
        format!("{}", key.time_parts),
        path_str_no_root(
            &key.file_path,
            &ctx.config.vaults[vault_name.as_ref()].root_dir,
        ),
    ));

    sleep(Duration::from_secs(60)).await;
}

fn get_non_existing(
    discovered: &HashSet<ReminderKey>,
    existing: &HashMap<ReminderKey, Option<JoinHandle<()>>>,
) -> Vec<ReminderKey> {
    existing
        .keys()
        .filter(|k| !discovered.contains(k))
        .cloned()
        .collect()
}
