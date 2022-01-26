use crate::{config::Config, db::Database};
use clap::Parser;
use eyre::WrapErr;

use std::path::PathBuf;

/// Init and/or update the cyndikator database
#[derive(Parser)]
pub struct Init {
    /// Config to load
    #[clap(short, long, env = "CYNDIKATOR_CONFIG")]
    config: Option<PathBuf>,

    /// Update the database schema
    #[clap(short, long)]
    update: bool,
}

impl Init {
    pub async fn run(self) -> eyre::Result<()> {
        let config = Config::load(self.config.as_deref())?;
        let mut db = Database::create(config.database_path()?)?;

        if self.update {
            db.migrate().wrap_err("unable to migrate")?;
        }

        Ok(())
    }
}
