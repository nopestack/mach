//
// TODO: revisit later
//
#![allow(unused)]
use std::collections::HashMap;

use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::exec::{task::Task, wasm_task_exec::WasmTaskExecutor};

use super::TaskExecutor;

type JobRx = mpsc::Receiver<Task>;

#[derive(Debug)]
pub struct ExecutionService {
    cancel_token: CancellationToken,
    db: HashMap<String, Vec<u8>>,
}

impl ExecutionService {
    pub fn new(cancel_token: CancellationToken) -> Self {
        //
        // TODO: initialize db from disk
        //
        ExecutionService {
            db: HashMap::new(),
            cancel_token,
        }
    }

    #[tracing::instrument(skip(self))]
    pub fn stop(&mut self) {
        tracing::info!("stopping execution service...");
        self.cancel_token.cancel();
        tracing::info!("stopped execution service");
    }

    #[tracing::instrument(skip(self, rx))]
    pub async fn run(&mut self, rx: JobRx) -> anyhow::Result<()> {
        tracing::info!("starting execution service...");

        // TODO: initialize a result collector task that writes results to db

        let task_executor = WasmTaskExecutor::new();
        tokio::spawn(run_task_processing_loop(
            rx,
            task_executor,
            self.cancel_token.clone(),
        ));

        tracing::info!("started execution service");

        Ok(())
    }
}

async fn run_task_processing_loop(
    mut rx: JobRx,
    mut executor: impl TaskExecutor,
    cancel_token: CancellationToken,
) {
    loop {
        tokio::select! {
        _ = cancel_token.cancelled() => {
            tracing::info!("stopping execution service");
            break;
        }
        Some(task) = rx.recv() => {
            handle_task(&mut executor, task);
            }
        }
    }
}

fn handle_task(executor: &mut impl TaskExecutor, task: Task) {
    tracing::info!("received task");

    match executor.exec(task) {
        Ok(task_result) => {
            tracing::info!("task result: {result:?}", result = task_result);
            // TODO: push to results db
        }
        Err(err) => {
            tracing::error!("error executing task: {}", err);
            // continue;
        }
    }
}
