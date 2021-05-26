use crate::db::Database;
use eyre::WrapErr;
use structopt::StructOpt;

use std::path::PathBuf;

/// Init and/or update the cyndikator database
#[derive(StructOpt)]
pub struct Init {
    /// where the database is located
    #[structopt(short, long, env = "CYNDIKATOR_DATABASE")]
    database: Option<String>,

    /// Update the database schema
    #[structopt(short, long)]
    update: bool,
}

impl Init {
    pub async fn run(self) -> eyre::Result<()> {
        let path = self
            .database
            .map_or_else(Database::default_path, PathBuf::from);
        let mut db = Database::create(path)?;

        if self.update {
            db.migrate().wrap_err("unable to migrate")?;
        }

        Ok(())
    }
}
