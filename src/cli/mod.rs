mod commands;
use crate::utils::error::Result;

use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Sungwoo Ha",
    about = "HubSpot Data Fetcher CLI"
)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: commands::Command,
}

pub async fn run() -> Result<()> {
    use clap::Parser;

    #[derive(Parser)]
    #[clap(
        version = "1.0",
        author = "Your Name",
        about = "HubSpot Data Fetcher CLI"
    )]
    struct Cli {
        #[clap(subcommand)]
        command: commands::Command,
    }

    let cli = Cli::parse();
    commands::execute(cli.command).await
}
