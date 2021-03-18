use crate::fetcher::Fetcher;
use eyre::WrapErr;
use structopt::StructOpt;
use tabular::{Row, Table};
use url::Url;

use crate::db::Database;
use std::path::PathBuf;

/// Start tracking a feed
#[derive(StructOpt)]
pub struct Track {
    /// where the database is located
    #[structopt(short, long, env = "CYNDIKATOR_DATABASE")]
    database: Option<String>,

    /// A rss feed to start  tracking
    feed: String,

    /// Override the ttl to fetch
    #[structopt(long)]
    ttl: Option<u32>,
}

impl Track {
    pub async fn run(self) -> eyre::Result<()> {
        let path = self
            .database
            .map_or_else(|| Database::default_path(), |s| PathBuf::from(s));
        let mut db = Database::open(path)?;

        let url = Url::parse(&self.feed).wrap_err("invalid url")?;

        let mut fetcher = Fetcher::new(&url);

        let title = fetcher.title().await?;

        db.track(&url, &title, self.ttl)?;

        Ok(())
    }
}

/// List feeds in the cydikator database
#[derive(StructOpt)]
pub struct Tracking {
    /// where the database is located
    #[structopt(short, long, env = "CYNDIKATOR_DATABASE")]
    database: Option<String>,
}

impl Tracking {
    pub async fn run(self) -> eyre::Result<()> {
        let path = self
            .database
            .map_or_else(|| Database::default_path(), |s| PathBuf::from(s));
        let mut db = Database::open(path)?;

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
#[derive(StructOpt)]
pub struct Untrack {
    /// where the database is located
    #[structopt(short, long, env = "CYNDIKATOR_DATABASE")]
    database: Option<String>,

    /// url to untrack
    feed: String,
}

impl Untrack {
    pub async fn run(self) -> eyre::Result<()> {
        let path = self
            .database
            .map_or_else(|| Database::default_path(), |s| PathBuf::from(s));
        let mut db = Database::open(path)?;

        let existed = db.untrack(&self.feed)?;

        if !existed {
            std::process::exit(1);
        }

        Ok(())
    }
}
