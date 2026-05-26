use clap::{Parser, Subcommand};

use crate::{Result, commands};

#[derive(Debug, Parser)]
#[command(name = "coderelic")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Scan(commands::scan::ScanArgs),
}

pub async fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Scan(args) => commands::scan::run(args).await,
    }
}
