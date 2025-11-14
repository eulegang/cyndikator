use clap::Parser;

use crate::cmd::Cli;

mod cmd;

trait Runner {
    async fn run(self) -> eyre::Result<()>;
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let cli = Cli::parse();

    cli.run().await?;

    Ok(())
}
