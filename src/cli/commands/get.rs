use std::net::SocketAddr;

use crate::api::client::Client as ApiClient;

#[derive(Debug, clap::Args)]
pub struct GetCmd {
    #[clap(long, default_value = "0.0.0.0:3401")]
    pub server_addr: SocketAddr,
    pub fn_id: uuid::Uuid,
}

#[tracing::instrument(skip(args))]
pub async fn exec(args: GetCmd) -> anyhow::Result<()> {
    let api_client = ApiClient::new()?;

    let response = api_client.get(args.server_addr, &args.fn_id).await?;

    let response_pretty = serde_json::to_string_pretty(&response)?;

    println!("{}", response_pretty);

    Ok(())
}
