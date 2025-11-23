use std::path::PathBuf;

use clap::Parser;
use cyndikator::Client;
use url::Url;

use crate::Runner;

#[derive(Parser)]
pub struct Eval {
    #[clap(short, long)]
    file: Option<PathBuf>,

    #[clap(short, long, default_value = "false")]
    all: bool,

    url: Url,
}

impl Runner for Eval {
    async fn run(self) -> eyre::Result<()> {
        let client = Client::builder().runtime_opt(self.file).build().await?;
        let feed = client.fetch_items(self.url).await?;

        let (_, items) = client.eval(feed).await?;
        for (item, prog) in items {
            if !prog.is_empty() || self.all {
                println!(
                    "[{}]({})",
                    item.title.as_deref().unwrap_or_default(),
                    item.id
                );
                println!("{}", prog);
            }
        }

        Ok(())
    }
}
