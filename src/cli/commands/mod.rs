pub mod call;
pub mod config;
pub mod delete;
pub mod get;
pub mod list;
pub mod serve;
pub mod upload;

#[derive(clap::Subcommand, Debug)]
pub enum SubCmd {
    /// Starts a Mach server
    Serve(serve::ServeCmd),

    /// Lists all available functions in the server
    List(list::ListCmd),

    /// Gets detailed information about a function
    Get(get::GetCmd),

    /// Calls a single WASM function
    Call(call::CallCmd),

    /// Upload a function to the server
    Upload(upload::UploadCmd),

    /// Deletes a function from the server
    Delete(delete::DeleteCmd),

    /// Manage Mach's configuration
    Config(config::ConfigCmd),
}
