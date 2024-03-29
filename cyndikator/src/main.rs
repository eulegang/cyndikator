use clap::Parser;
use std::env;

mod cli;
mod config;
mod daemon;
mod db;
mod fetcher;
mod view;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    if env::var("CYND_LOG").is_err() {
        env::set_var("CYND_LOG", "info");
    }

    pretty_env_logger::init_custom_env("CYND_LOG");

    let cli = cli::Cli::parse();

    cli.run().await?;

    Ok(())
}
