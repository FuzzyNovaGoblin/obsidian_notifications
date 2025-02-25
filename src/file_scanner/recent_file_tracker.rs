use crate::{
    bot::send_msgs::{report_reminder_notification, report_rust_error},
    file_scanner::path_str_no_root,
    state::State,
};
use chrono::prelude::*;
use regex::Regex;
use shared_singleton::Singleton;
use std::{
    borrow::BorrowMut,
    collections::{HashMap, HashSet},
    fs,
    path::PathBuf,
    sync::Arc,
    time::Duration,
};
use tokio::{spawn, task::JoinHandle, time::sleep};

const SLEEP_DURATION: Duration = Duration::from_secs(5 * 60);

pub async fn scan_file_activity(ctx: crate::Ctx, vault_name: Arc<String>) {
    let vault = &ctx.config.vaults[&*vault_name];

    loop {
        let mut file_list = Vec::new();

        let mut path_queue: Vec<PathBuf> = vec![vault.root_dir.clone().into()];
        sleep(SLEEP_DURATION).await;

        while let Some(p) = path_queue.pop() {
            let paths = std::fs::read_dir(p).unwrap();
            for entry in paths.map(|v| v.unwrap()) {
                let entry_type = entry.file_type().unwrap();
                if entry_type.is_dir(){
                    path_queue.push(entry.path());
                }else if entry_type.is_file() {
                    file_list.push(entry.path());
                }
            }
        }
        // let l = file_list[0];
        file_list.sort_by(|a,b|a.metadata().unwrap().created().unwrap().cmp(&b.metadata().unwrap().created().unwrap()));
        println!("{:#?}", file_list);

    }
}
