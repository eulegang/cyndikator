use clap::Parser;
use cyndikator::Client;
use url::Url;

use crate::Runner;

#[derive(Parser)]
pub struct Track {
    url: Url,

    #[clap(short, long)]
    ttl: Option<u32>,
}

impl Runner for Track {
    async fn run(self) -> eyre::Result<()> {
        Client::builder()
            .migrate()
            .build()
            .await?
            .track(self.url, self.ttl)
            .await?;

        Ok(())
    }
}
