use clap::{Parser, Subcommand};

mod containers;

use crate::containers::{build_container_image, BuildContainerImageArgs};

/// The `xtask` CLI.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Subcommands
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build container image.
    BuildContainerImage(BuildContainerImageArgs),
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::BuildContainerImage(args) => {
            build_container_image(args)?;
        }
    }

    Ok(())
}
