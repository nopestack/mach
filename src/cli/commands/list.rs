use std::net::SocketAddr;

use crate::api::client::Client as ApiClient;

#[derive(Debug, clap::Args)]
pub struct ListCmd {
    #[clap(long, default_value = "0.0.0.0:3401")]
    pub server_addr: SocketAddr,
}

#[tracing::instrument(skip(args))]
pub async fn exec(args: ListCmd) -> anyhow::Result<()> {
    let api_client = ApiClient::new()?;

    let response = api_client.list(args.server_addr).await?;

    let response_pretty = serde_json::to_string_pretty(&response)?;

    println!("{}", response_pretty);
    Ok(())
}
