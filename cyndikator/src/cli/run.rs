use crate::daemon::Daemon;
use crate::db::Database;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

use eyre::WrapErr;

use cyndikator_dispatch::Dispatch;

#[derive(StructOpt)]
pub struct Run {
    #[structopt(short, long, env = "CYNDIKATOR_DATABASE")]
    database: Option<String>,

    #[structopt(short, long)]
    file: Option<String>,
}

impl Run {
    pub async fn run(self) -> eyre::Result<()> {
        let path = self
            .database
            .as_ref()
            .map_or_else(|| Database::default_path(), |s| PathBuf::from(s));
        let db = Database::open(path)?;

        let dispatch_filepath = self
            .file
            .map(|file| PathBuf::from(file))
            .unwrap_or_else(|| {
                let mut p = Database::default_path();
                p.pop();
                p.push("cyndikator.dispatch");

                p
            });

        let content = fs::read_to_string(dispatch_filepath)?;

        let dispatch = Dispatch::parse(&content).wrap_err("failed to parse dispatch file")?;

        Daemon::new(db, dispatch, 60).run().await
    }
}
