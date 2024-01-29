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

        commands::SubCmd::Upload(args) => {
            commands::upload::exec(args).await
        }

        commands::SubCmd::Get(args) => {
            commands::get::exec(args).await
        }

        commands::SubCmd::List(args) => {
            commands::list::exec(args).await
        }

        commands::SubCmd::Call(args) => {
            commands::call::exec(args).await
        }


        commands::SubCmd::Delete(args) => {
            commands::delete::exec(args).await
        }

        commands::SubCmd::Config(args) => {
            commands::config::exec(args).await
        }

        _ => {
            anyhow::bail!("Not supported yet");
        } 
    }
}
