use structopt::StructOpt;

mod cli;
mod db;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let cli = cli::Cli::from_args();

    cli.run().await?;

    Ok(())
}
