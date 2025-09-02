use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::env::{PKG_NAME, PKG_RELEASE};

mod read;
mod run;

#[derive(Debug, Parser)]
#[command(name = PKG_NAME, bin_name = PKG_NAME)]
#[command(version = PKG_RELEASE)]
#[command(disable_version_flag = false, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Run a program and inject secrets")]
    Run(run::RunCommandArguments),

    #[command(about = "Read a secret")]
    Read(read::ReadCommandArguments),
}

pub async fn init() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Run(args) => run::init(args).await,
        Commands::Read(args) => read::init(args).await,
    }
}
