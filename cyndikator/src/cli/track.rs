use eyre::WrapErr;
use structopt::StructOpt;
use url::Url;

use crate::db::Database;
use std::path::PathBuf;

use cyndikator_rss::Rss;

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

        let rss = Rss::fetch(&url)
            .await
            .wrap_err("unable to fetch rss feed")?;

        db.track(&url, &rss, self.ttl)?;

        Ok(())
    }
}
