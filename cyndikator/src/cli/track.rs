use crate::fetcher::Fetcher;
use clap::Parser;
use eyre::WrapErr;
use tabular::{Row, Table};
use url::Url;

use crate::config::Config;
use std::path::PathBuf;

use super::db_coord;

/// Start tracking a feed
#[derive(Parser)]
pub struct Track {
    /// Config to load
    #[clap(short, long, env = "CYNDIKATOR_CONFIG")]
    config: Option<PathBuf>,

    /// A rss feed to start  tracking
    feed: String,

    /// Override the ttl to fetch
    #[clap(long)]
    ttl: Option<u32>,
}

impl Track {
    pub async fn run(self) -> eyre::Result<()> {
        let config = Config::load(self.config.as_deref())?;
        let mut db = db_coord(&config.database)?.open()?;

        let url = Url::parse(&self.feed).wrap_err("invalid url")?;

        let mut fetcher = Fetcher::new(&url);

        let title = fetcher.title().await?;

        db.track(&url, &title, self.ttl)?;

        Ok(())
    }
}

/// List feeds in the cydikator database
#[derive(Parser)]
pub struct Tracking {
    /// Config to load
    #[clap(short, long, env = "CYNDIKATOR_CONFIG")]
    config: Option<PathBuf>,
}

impl Tracking {
    pub async fn run(self) -> eyre::Result<()> {
        let config = Config::load(self.config.as_deref())?;
        let mut db = db_coord(&config.database)?.open()?;

        let feeds = db.tracking()?;

        let mut table = Table::new("{:<} {:<} {:<} {:<}");

        table.add_row(
            Row::new()
                .with_cell("title")
                .with_cell("ttl")
                .with_cell("last_fetch")
                .with_cell("url"),
        );

        for feed in feeds {
            table.add_row(
                Row::new()
                    .with_cell(feed.title)
                    .with_cell(feed.ttl.unwrap_or(60))
                    .with_cell(
                        feed.last_fetch
                            .map_or_else(|| "never".to_string(), |d| d.to_rfc3339()),
                    )
                    .with_cell(feed.url),
            );
        }

        println!("{}", table);

        Ok(())
    }
}

/// Remove feed from being tracked
#[derive(Parser)]
pub struct Untrack {
    /// Config to load
    #[clap(short, long, env = "CYNDIKATOR_CONFIG")]
    config: Option<PathBuf>,

    /// url to untrack
    feed: String,
}

impl Untrack {
    pub async fn run(self) -> eyre::Result<()> {
        let config = Config::load(self.config.as_deref())?;
        let mut db = db_coord(&config.database)?.open()?;

        let existed = db.untrack(&self.feed)?;

        if !existed {
            std::process::exit(1);
        }

        Ok(())
    }
}
