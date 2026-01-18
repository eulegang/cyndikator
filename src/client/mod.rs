use crate::db::Conn;
use chrono::Utc;
use url::Url;

use crate::{
    FeedItem, Result,
    client::daemon::Daemon,
    feed::{Feed, FeedMeta},
    interp::{Interp, Program},
    runtime::Runtime,
};

mod builder;
mod daemon;

pub use builder::ClientBuilder;

pub struct Client {
    runtime: Runtime,
    conn: Conn,
    fetcher: crate::fetcher::Fetcher,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    pub async fn fetch_items(&self, url: Url) -> Result<Feed> {
        self.fetcher.fetch_items(url).await
    }

    pub async fn eval(&self, feed: Feed) -> Result<(FeedMeta, Vec<(FeedItem, Program)>)> {
        let mut res = Vec::new();
        for item in feed.items {
            let prog = self
                .runtime
                .process(feed.meta.clone(), item.clone())
                .await?;

            res.push((item, prog));
        }

        Ok((feed.meta, res))
    }

    pub async fn untrack(&self, url: Url, purge: bool) -> crate::Result<()> {
        self.conn.untrack(url.to_string(), purge).await?;
        Ok(())
    }

    pub async fn track(&self, url: Url, ttl: Option<u32>) -> crate::Result<()> {
        let endpoint = url.to_string();
        let feed = self.fetch_items(url).await?;

        self.conn.track(endpoint, Utc::now()).await?;
        let (meta, instructions) = self.eval(feed).await?;

        let interp = Interp {};
        for (item, prog) in instructions {
            interp.run(&meta, &item, &prog)?;
        }

        Ok(())
    }

    pub fn daemon(self) -> Daemon {
        Daemon::new(self)
    }
}
