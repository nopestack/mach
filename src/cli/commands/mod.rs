pub mod serve;

#[derive(clap::Subcommand, Debug)]
pub enum SubCmd {
    /// Starts a Mach server
    Serve(serve::ServeCmd),

    /// Lists all available functions in the server
    List(client::ListCmd),

    /// Gets detailed information about a function
    Get(client::GetCmd),

    /// Calls a single WASM function
    Call(call::CallCmd),

    /// Upload a function to the server
    Upload(client::UploadCmd),

    /// Deletes a function from the server
    Delete(client::DeleteCmd),

    /// Manage the CLI's configuration
    Config(config::ConfigCmd),
}

mod call {
    use std::net::SocketAddr;

    #[derive(Debug, clap::Args)]
    pub struct CallCmd {
        #[clap(long, default_value = "0.0.0.0:3401")]
        pub server_addr: SocketAddr,
        pub name: String,
    }
}

mod config {
    #[derive(Debug, clap::Args)]
    pub struct ConfigCmd {
        pub path: String,
    }
}

mod client {
    use std::{net::SocketAddr, path::PathBuf};

    #[derive(Debug, clap::Args)]
    pub struct ListCmd {
        #[clap(long, default_value = "0.0.0.0:3401")]
        pub server_addr: SocketAddr,
    }

    #[derive(Debug, clap::Args)]
    pub struct GetCmd {
        #[clap(long, default_value = "0.0.0.0:3401")]
        pub server_addr: SocketAddr,
        pub fn_id: String,
    }

    #[derive(Debug, clap::Args)]
    pub struct UploadCmd {
        #[clap(long, default_value = "0.0.0.0:3401")]
        pub server_addr: SocketAddr,
        pub fn_id: String,
        pub fn_file: PathBuf,
    }

    #[derive(Debug, clap::Args)]
    pub struct DeleteCmd {
        #[clap(long, default_value = "0.0.0.0:3401")]
        pub server_addr: SocketAddr,
        pub fn_id: String,
    }
}
