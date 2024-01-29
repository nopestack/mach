#[derive(Debug, clap::Args)]
pub struct ConfigCmd {
    pub path: String,
}

#[tracing::instrument(skip(args))]
pub async fn exec(args: ConfigCmd) -> anyhow::Result<()> {
    Ok(())
}
