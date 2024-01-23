use std::net::SocketAddr;

use crate::storage::FnId;

#[derive(Debug, clap::Args)]
pub struct CallCmd {
    #[clap(long, default_value = "0.0.0.0:3401")]
    pub server_addr: SocketAddr,

    #[clap(long)]
    pub fn_id: FnId,

    #[clap(default_value = "")]
    pub args: String,
}

pub async fn exec(args: CallCmd) -> anyhow::Result<()> {
    let mut api_client = crate::api::client::Client::new()?;
    let server_addr = args.server_addr;
    let fn_id = args.fn_id;
    let fn_args = args.args.as_bytes().to_vec();

    let response = api_client.call(server_addr, &fn_id, fn_args).await?;
    let response_json = serde_json::to_string_pretty(&response)?;
    println!("{}", response_json);

    Ok(())
}
