use crate::config::Config;
use crate::daemon::Daemon;
use clap::Parser;

use std::path::PathBuf;

use super::{db_coord, dispatch_coord};

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
        let dispatch = dispatch_coord(&config.dispatch)?.dispatcher()?;

        Daemon::new(db, dispatch, 60).run().await
    }
}
