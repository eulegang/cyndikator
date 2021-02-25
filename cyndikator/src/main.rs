use structopt::StructOpt;

mod cli;
mod daemon;
mod db;
mod ticker;
mod tracker;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let cli = cli::Cli::from_args();

    cli.run().await?;

    Ok(())
}
