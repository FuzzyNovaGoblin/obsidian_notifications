use std::{fs, path::PathBuf, sync::Arc, thread::sleep};

use chrono::{Datelike, Local, TimeDelta, TimeZone};
use regex::Regex;
use tokio::spawn;

use crate::{bot::send_msgs::send_daily_todo_reminder, config::vault::Vault};
const HEADER_REGEX_PT1: &str = r#"(?<todo_header>^# "#;
const HEADER_REGEX_PT2: &str = r#"$)|(^# [A-Za-z0-9\s]*$)"#;
// const HEADER_REGEX: &str = r#"(?<todo_header>^# TODO$)|(^# [A-Za-z0-9\s]*$)"#;
const TASK_REGEX: &str = r#"^(?:- (?<checkbox>\[(?<checked>.)])?)\s?(?<message>.*)"#;

pub async fn daily_todo_thread(ctx: crate::Ctx, vault_name: Arc<String>) {
    let vault = &ctx.config.vaults[&*vault_name];
    let (task_regex, header_regex) = build_regex(&vault);

    loop {
        let tasks = get_reminders(vault, &task_regex, &header_regex);
        let mut msg = String::new();

        for t in tasks {
            if !t.checked {
                msg += &format!("- {}\n", t.msg);
            }
        }

        send_daily_todo_reminder(
            ctx.clone(),
            msg,
            ctx.config.vaults[vault_name.as_ref()].destination.clone(),
        )
        .await;

        let now = Local::now();
        let target_time = chrono::Local
            .with_ymd_and_hms(now.year(), now.month(), now.day(), 9, 0, 0)
            .single()
            .unwrap()
            - now;

        let sleep_time = if target_time.num_seconds() < 0 {
            (target_time + TimeDelta::days(1)).to_std().unwrap()
        } else {
            target_time.to_std().unwrap()
        };
        sleep(sleep_time);
    }
}

fn get_reminders(vault: &Vault, task_regex: &Regex, header_regex: &Regex) -> Vec<Task> {
    let file_data = fs::read_to_string(
        PathBuf::from(&vault.root_dir).join(PathBuf::from(vault.todo_path.trim_start_matches("/"))),
    )
    .expect("failed to read TODO file, did you set the correct value in your config?");

    let mut tasks = Vec::new();
    let mut under_todo_header = false;

    for line in file_data.lines() {
        if header_regex.is_match(line) {
            if header_regex
                .captures(line)
                .unwrap()
                .name("todo_header")
                .is_some()
            {
                under_todo_header = true;
            } else {
                under_todo_header = false;
            }
        } else if under_todo_header {
            if let Some(caps) = task_regex.captures(line) {
                let checked_cap = caps.name("checked");
                tasks.push(Task {
                    checked: checked_cap.is_none()
                        || (checked_cap.is_some()
                            && checked_cap.unwrap().as_str().trim().len() > 0),
                    msg: match caps.name("message") {
                        Some(v) => v.as_str().into(),
                        None => String::new(),
                    },
                });
            }
        }
    }

    tasks
}

fn build_regex(vault: &Vault) -> (Regex, Regex) {
    let task_regex = Regex::new(TASK_REGEX).expect("failed to compile RegEx TASK_REGEX");
    let header_regex = Regex::new(&format!(
        "{HEADER_REGEX_PT1}{}{HEADER_REGEX_PT2}",
        vault.todo_section_header
    ))
    .expect("failed to compile RegEx HEADER_REGEX");

    (task_regex, header_regex)
}

#[derive(Debug, Clone)]
struct Task {
    checked: bool,
    msg: String,
}
