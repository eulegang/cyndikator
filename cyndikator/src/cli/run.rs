use crate::config::Config;
use crate::daemon::Daemon;
use clap::Parser;
use eyre::{ContextCompat, WrapErr};
use std::fs;
use std::path::PathBuf;

use cyndikator_dispatch::Dispatch;

use super::db_coord;

/// Start tracking feeds
#[derive(Parser)]
pub struct Run {
    /// Config to load
    #[clap(short, long, env = "CYNDIKATOR_CONFIG")]
    config: Option<PathBuf>,
}

impl Run {
    pub async fn run(self) -> eyre::Result<()> {
        let config = Config::load(self.config.as_deref())?;
        let db = db_coord(&config.database)?.open()?;
        let dispatch_path = config.dispatch.path.wrap_err("dispatch path not set")?;

        let content = fs::read_to_string(dispatch_path)?;

        let dispatch = Dispatch::parse(&content).wrap_err("failed to parse dispatch file")?;

        Daemon::new(db, dispatch, 60).run().await
    }
}
