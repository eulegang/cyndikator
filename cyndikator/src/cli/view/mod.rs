use crate::{config::Config, db::Database};
use clap::Parser;
use std::path::PathBuf;

/// View recorded events
#[derive(Parser)]
pub struct View {
    /// Config to load
    #[clap(short, long, env = "CYNDIKATOR_CONFIG")]
    config: Option<PathBuf>,
}

impl View {
    pub async fn run(self) -> eyre::Result<()> {
        let config = Config::load(self.config.as_deref())?;
        let db = Database::open(config.database_path()?)?;
        let view = crate::view::View::new(db);

        view.interact()
    }
}
