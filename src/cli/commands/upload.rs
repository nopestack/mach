use std::{net::SocketAddr, path::PathBuf};

use crate::api::client::Client as ApiClient;

#[derive(Debug, clap::Args)]
pub struct UploadCmd {
    #[clap(long, default_value = "0.0.0.0:3401")]
    pub server_addr: SocketAddr,
    pub fn_file: PathBuf,
}

#[tracing::instrument(skip(args))]
pub async fn exec(args: UploadCmd) -> anyhow::Result<()> {
    let mut api_client = ApiClient::new()?;

    let filename = args
        .fn_file
        .file_name()
        .ok_or(anyhow::format_err!("invalid filename"))?
        .to_str()
        .ok_or(anyhow::format_err!("invalid filename"))?;

    let file_data = tokio::fs::read(&args.fn_file).await?;

    let response = api_client
        .upload(args.server_addr, filename, file_data)
        .await?;

    crate::cli::pretty_print(response)
}
