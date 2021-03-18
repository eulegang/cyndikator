use std::env;
use structopt::StructOpt;

mod cli;
mod daemon;
mod db;
mod fetcher;
mod ticker;
mod tracker;
mod view;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    if env::var("CYND_LOG").is_err() {
        env::set_var("CYND_LOG", "info");
    }

    pretty_env_logger::init_custom_env("CYND_LOG");

    let cli = cli::Cli::from_args();

    cli.run().await?;

    Ok(())
}
