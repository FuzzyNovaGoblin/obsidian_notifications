use bot::{bot_start, context::DisCtx};
use file_scanner::{
    file_search_config::FileSearchConfig,
    sync_conflict::{self},
    time_reminder,
};
use std::time::Duration;
use tokio::{spawn, task::JoinHandle, time::sleep};

use crate::bot::send_msgs::report_rust_error;

pub mod bot;
pub mod file_scanner;

#[tokio::main]
async fn main() {
    let (mut tasks, dis_ctx) = start_tasks().await;

    while !tasks.is_empty() {
        tokio::time::sleep(Duration::from_secs(10)).await;
        if tasks.len() == 1 {
            break;
        }
        let done = tasks
            .iter()
            .enumerate()
            .rev()
            .filter(|(_, t)| t.is_finished())
            .map(|(i, _)| i)
            .collect::<Vec<_>>();

        for t in done.into_iter().map(|i| tasks.remove(i)) {
            if let Err(e) = t.await {
                let err_msg = format!("thread failed: {:?}", e);
                eprintln!("{err_msg}");
                report_rust_error(dis_ctx.clone(), err_msg).await;
            }
        }
    }
}

async fn start_tasks() -> (Vec<JoinHandle<()>>, DisCtx) {
    let (dis_ctx, mut client) = bot_start().await;

    let mut tasks = vec![spawn(async move {
        client.start().await.unwrap();
    })];
    sleep(Duration::from_secs(1)).await;

    // for config in FileSearchConfig::gen_all_debug_configs() {
    // for config in FileSearchConfig::gen_short_debug_config() {
    for config in FileSearchConfig::gen_all_configs() {
        tasks.push(spawn(sync_conflict::look_for_sync_conflicts(
            dis_ctx.clone(),
            config.clone(),
        )));

        tasks.push(spawn(time_reminder::look_for_time_reminders(
            dis_ctx.clone(),
            config.clone(),
        )));
    }

    (tasks, dis_ctx)
}
