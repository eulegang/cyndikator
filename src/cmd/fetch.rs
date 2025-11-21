use clap::Parser;
use cyndikator::Client;
use url::Url;

use crate::Runner;

#[derive(Parser)]
pub struct Fetch {
    url: Url,
}

impl Runner for Fetch {
    async fn run(self) -> eyre::Result<()> {
        let feed = Client::builder()
            .build()
            .await?
            .fetch_items(self.url)
            .await?;

        serde_json::to_writer_pretty(std::io::stdout(), &feed)?;

        Ok(())
    }
}
