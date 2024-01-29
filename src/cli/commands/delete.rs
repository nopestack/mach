use std::net::SocketAddr;

#[derive(Debug, clap::Args)]
pub struct DeleteCmd {
    #[clap(long, default_value = "0.0.0.0:3401")]
    pub server_addr: SocketAddr,
    pub fn_id: uuid::Uuid,
}

#[tracing::instrument(skip(args))]
pub async fn exec(args: DeleteCmd) -> anyhow::Result<()> {
    Ok(())
}
