use crate::{config::Config, db::Database};
use std::path::PathBuf;
use structopt::StructOpt;

/// View recorded events
#[derive(StructOpt)]
pub struct View {
    /// Config to load
    #[structopt(short, long, env = "CYNDIKATOR_CONFIG")]
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
