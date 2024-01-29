pub mod app;
mod commands;
pub use app::*;

/// Formats serializable server responses into JSON and prints them to stdout
pub(crate) fn pretty_print<T>(response: T) -> anyhow::Result<()>
where
    T: serde::Serialize,
{
    let response_pretty = serde_json::to_string_pretty(&response)?;

    println!("{}", response_pretty);

    Ok(())
}
