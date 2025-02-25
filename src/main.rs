use crate::bot::send_msgs::report_rust_error;
use bot::bot_start;
use config::Config;
use file_scanner::{daily_todo, sync_conflict, time_reminder};
use std::{sync::Arc, time::Duration};
use tokio::{spawn, task::JoinHandle, time::sleep};

pub mod bot;
pub mod config;
pub mod file_scanner;
pub mod state;

type Ctx = Arc<config::Context>;

#[tokio::main]
async fn main() {
    let config = Config::load();
    let (mut tasks, ctx) = start_tasks(config).await;

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
                report_rust_error(ctx.clone(), err_msg).await;
            }
        }
    }
}

async fn start_tasks(config: Config) -> (Vec<JoinHandle<()>>, Ctx) {
    let (ctx, mut client) = bot_start(config).await;
    let ctx = Arc::new(ctx);

    let mut tasks = vec![spawn(async move {
        client.start().await.unwrap();
    })];
    sleep(Duration::from_secs(1)).await;

    for vault_name in ctx.get_all_vault_names() {
        tasks.push(spawn(sync_conflict::look_for_sync_conflicts(
            ctx.clone(),
            vault_name.clone(),
        )));

        tasks.push(spawn(time_reminder::look_for_time_reminders(
            ctx.clone(),
            vault_name.clone().clone(),
        )));

        tasks.push(spawn(daily_todo::daily_todo_thread(
            ctx.clone(),
            vault_name.clone().clone(),
        )));
    }

    (tasks, ctx)
}
