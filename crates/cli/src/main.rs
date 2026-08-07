use anyhow::Result;
use clap::{Parser, Subcommand};

mod install;
mod policy;

#[derive(Parser)]
#[command(name = "wrdn")]
#[command(about = "Safely run third party code as wasm modules", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install a third-party wasm module with the wrdn virtualizer
    Install {
        /// OCI reference to the guest wasm component (e.g. ghcr.io/org/package:tag)
        oci_ref: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .format_module_path(false)
        .init();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Install { oci_ref } => {
            install::run_install(oci_ref).await?;
        }
    }

    Ok(())
}
