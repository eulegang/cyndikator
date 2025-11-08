use std::path::PathBuf;

use clap::Parser;
use cyndikator::Client;
use url::Url;

#[derive(Parser)]
pub enum Cli {
    Eval(Eval),
}

#[derive(Parser)]
pub struct Eval {
    #[clap(short, long)]
    file: Option<PathBuf>,

    url: Url,
}

trait Runner {
    async fn run(self, client: Client) -> eyre::Result<()>;
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let cli = Cli::parse();

    let client = Client::new();

    cli.run(client).await?;

    Ok(())
}

impl Runner for Cli {
    async fn run(self, client: Client) -> eyre::Result<()> {
        match self {
            Cli::Eval(eval) => eval.run(client).await,
        }
    }
}

impl Runner for Eval {
    async fn run(self, client: Client) -> eyre::Result<()> {
        let feed = client.fetch_items(self.url).await?;

        dbg!(feed);

        todo!()
    }
}
