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

    #[clap(short, long, default_value = "false")]
    all: bool,

    url: Url,
}

trait Runner {
    async fn run(self) -> eyre::Result<()>;
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let cli = Cli::parse();

    cli.run().await?;

    Ok(())
}

impl Runner for Cli {
    async fn run(self) -> eyre::Result<()> {
        match self {
            Cli::Eval(eval) => eval.run().await,
        }
    }
}

impl Runner for Eval {
    async fn run(self) -> eyre::Result<()> {
        let client = Client::new(self.file)?;
        let feed = client.fetch_items(self.url).await?;

        for item in &feed.items {
            let res = client.eval(item.clone()).await?;
            if !res.is_empty() || self.all {
                println!(
                    "[{}]({})",
                    item.title.as_deref().unwrap_or_default(),
                    item.id
                );
                println!("{}", res);
            }
        }

        //dbg!(feed);

        Ok(())
    }
}
