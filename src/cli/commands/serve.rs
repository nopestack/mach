use std::net::SocketAddr;

use crate::storage::{backends::local::LocalBackend, DATA_DIR};

#[derive(Debug, clap::Args)]
pub struct ServeCmd {
    #[clap(long, default_value = DATA_DIR)]
    pub path: String,
    #[clap(long, default_value = "0.0.0.0:3401")]
    pub addr: SocketAddr,
}

#[tracing::instrument(skip(args))]
pub async fn exec(args: ServeCmd) -> anyhow::Result<()> {
    let addr = args.addr;
    let path = args.path.into();

    let storage_backend = LocalBackend::new(path)?;
    let wasm_task_exec = crate::exec::wasm_task_exec::WasmTaskExecutor::new();

    let mut server = crate::api::Server::new(addr, wasm_task_exec, storage_backend);

    server.run().await?;

    crate::listen_for_ctrl_c().await?;

    tracing::info!("shutting down");

    server.stop();

    tracing::info!("shutdown complete");

    Ok(())
}
