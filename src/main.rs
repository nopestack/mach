use mach::setup_tracing;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_tracing();
    mach::cli::exec().await
}
