pub mod api;
pub mod cli;
pub mod exec;
pub mod node;
pub mod storage;

pub const JOB_BUFFER_SIZE: usize = 100;

pub fn setup_tracing() {
    tracing_subscriber::fmt()
        .compact()
        .with_target(false)
        .init();
}

pub async fn listen_for_ctrl_c() -> anyhow::Result<()> {
    match tokio::signal::ctrl_c().await {
        Ok(()) => {}
        Err(err) => {
            tracing::error!("Error listening for ctrl-c: {err}");
        }
    }
    Ok(())
}
