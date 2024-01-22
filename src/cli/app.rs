use clap::Parser;

use super::commands;

/// Run tiny WASM functions at Mach speed
#[derive(clap::Parser, Debug)]
#[clap(
    author, 
    version, 
    about, 
    long_about = None, 
    arg_required_else_help(true)
)]
pub struct App {
    #[command(subcommand)]
    commands: commands::SubCmd,
}

pub async fn exec() -> anyhow::Result<()> {
    let app = App::parse();

    match app.commands {
        commands::SubCmd::Serve(args) => {
            commands::serve::exec(args).await
        }
        _ => {
            anyhow::bail!("Not supported yet");
        } 
    }
}
