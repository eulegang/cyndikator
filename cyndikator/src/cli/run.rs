use crate::daemon::Daemon;
use crate::{config::Config, db::Database};
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

use eyre::WrapErr;

use cyndikator_dispatch::Dispatch;

/// Start tracking feeds
#[derive(StructOpt)]
pub struct Run {
    /// Config to load
    #[structopt(short, long, env = "CYNDIKATOR_CONFIG")]
    config: Option<PathBuf>,
}

impl Run {
    pub async fn run(self) -> eyre::Result<()> {
        let config = Config::load(self.config.as_deref())?;
        let db_path = config.database_path()?;
        let db = Database::open(db_path)?;
        let dispatch_path = config.dispatch_path()?;

        let content = fs::read_to_string(dispatch_path)?;

        let dispatch = Dispatch::parse(&content).wrap_err("failed to parse dispatch file")?;

        Daemon::new(db, dispatch, 60).run().await
    }
}
