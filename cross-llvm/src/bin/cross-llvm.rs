use clap::{Parser, Subcommand};

use cross_llvm::run::{run, Run};

/// Containerized (cross, but not only) LLVM toolchains.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Subcommands
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a custom command.
    Run(Run),
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run(args) => run(args),
    }
}
