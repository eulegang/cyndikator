use cyndikator::Client;

use crate::Runner;

#[derive(clap::Parser)]
pub struct Run {}

impl Runner for Run {
    async fn run(self) -> eyre::Result<()> {
        let client = Client::builder().build()?;

        client.migrate().await?;

        client.run().await?;

        Ok(())
    }
}
