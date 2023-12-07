use std::time::Duration;
use bot::{bot_start, context::DisCtx};
use file_scanner::sync_conflict::{self, SyncConflictConfig};
use tokio::{spawn, task::JoinHandle};

pub mod bot;
pub mod file_scanner;

#[tokio::main]
async fn main() {
    let (mut tasks, dis_ctx) = start_tasks().await;

    while !tasks.is_empty() {
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
                bot::report_rust_error(dis_ctx.clone(), err_msg).await;
            }
        }
        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}

async fn start_tasks() -> (Vec<JoinHandle<()>>, DisCtx) {
    let (dis_ctx, mut client) = bot_start().await;

    let mut tasks = vec![spawn(async move {
        client.start().await.unwrap();
    })];

    for config in SyncConflictConfig::gen_all_configs() {
        tasks.push(spawn(sync_conflict::look_for_sync_conflicts(
            dis_ctx.clone(),
            config,
        )))
    }

    (tasks, dis_ctx)
}
