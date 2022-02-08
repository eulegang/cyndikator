use clap::Parser;
use cyndikator_dispatch::DispatcherSource;
use eyre::ContextCompat;

use crate::{
    config::{DatabaseConfig, DispatchConfig},
    db::DatabaseCoord,
};

mod init;
mod preview;
mod run;
mod track;
mod view;

#[derive(Parser)]
pub enum Cli {
    Run(run::Run),
    Track(track::Track),
    Tracking(track::Tracking),
    Untrack(track::Untrack),
    Init(init::Init),
    View(view::View),
    Preview(preview::Preview),
}

impl Cli {
    pub async fn run(self) -> eyre::Result<()> {
        match self {
            Cli::Run(cmd) => cmd.run().await,
            Cli::Track(cmd) => cmd.run().await,
            Cli::Tracking(cmd) => cmd.run().await,
            Cli::Untrack(cmd) => cmd.run().await,
            Cli::Init(cmd) => cmd.run().await,
            Cli::View(cmd) => cmd.run().await,
            Cli::Preview(cmd) => cmd.run().await,
        }
    }
}

fn db_coord(cfg: &DatabaseConfig) -> eyre::Result<DatabaseCoord> {
    match cfg.ty.as_str() {
        "sqlite3" => {
            let path = cfg
                .path
                .as_ref()
                .wrap_err("missing sqlite3 database path")?;

            Ok(DatabaseCoord::Sqlite(path))
        }

        ty => eyre::bail!("invalid database driver type: {}", ty),
    }
}

fn dispatch_coord(cfg: &DispatchConfig) -> eyre::Result<DispatcherSource> {
    match cfg.ty.as_str() {
        "dispatch" => {
            let path = cfg.path.as_ref().wrap_err("missing dispatch path")?;

            Ok(DispatcherSource::Dispatch(path))
        }

        "lua" => {
            let path = cfg.path.as_ref().wrap_err("missing lua path")?;

            Ok(DispatcherSource::Lua(path))
        }

        ty => eyre::bail!("invalid dispatch type: {}", ty),
    }
}
