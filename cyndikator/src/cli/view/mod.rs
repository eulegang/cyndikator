use crate::db::Database;
use std::path::PathBuf;
use structopt::StructOpt;

/// View recorded events
#[derive(StructOpt)]
pub struct View {
    #[structopt(short, long, env = "CYNDIKATOR_DATABASE")]
    database: Option<String>,
}

impl View {
    pub async fn run(self) -> eyre::Result<()> {
        let path = self
            .database
            .as_ref()
            .map_or_else(Database::default_path, PathBuf::from);
        let db = Database::open(path)?;

        let view = crate::view::View::new(db);

        view.interact()
    }
}
