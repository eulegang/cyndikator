use clap::Parser;
use cyndikator::Client;
use url::Url;

use crate::Runner;

#[derive(Parser)]
pub struct Untrack {
    url: Url,

    #[clap(short, long)]
    purge: bool,
}

impl Runner for Untrack {
    async fn run(self) -> eyre::Result<()> {
        let client = Client::builder().build()?;

        client.migrate().await?;
        client.untrack(self.url, self.purge).await?;

        Ok(())
    }
}
