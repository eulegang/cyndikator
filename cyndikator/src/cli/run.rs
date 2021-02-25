use crate::daemon::Daemon;
use crate::db::Database;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Run {
    #[structopt(short, long, env = "CYNDIKATOR_DATABASE")]
    database: Option<String>,
}

impl Run {
    pub async fn run(self) -> eyre::Result<()> {
        let path = self
            .database
            .as_ref()
            .map_or_else(|| Database::default_path(), |s| PathBuf::from(s));
        let db = Database::open(path)?;

        Daemon::new(db, 60).run().await
    }
}
