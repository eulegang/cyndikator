use cyndikator::Client;

use crate::Runner;

#[derive(clap::Parser)]
pub struct Run {}

impl Runner for Run {
    async fn run(self) -> eyre::Result<()> {
        Client::builder().migrate().build().await?.run().await?;
        Ok(())
    }
}
