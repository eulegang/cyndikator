use chrono::{Duration, Utc};
use rusqlite::Connection;
use url::Url;

use crate::{
    FeedItem, Result, client::db::DBOperation, feed::Feed, interp::inst::Program, runtime::Runtime,
};

mod builder;
mod db;

pub use builder::ClientBuilder;

pub struct Client {
    client: reqwest::Client,
    runtime: Runtime,
    conn: Connection,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    pub async fn fetch_items(&self, url: Url) -> Result<Feed> {
        let resp = self
            .client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        let bare_feed = feed_rs::parser::parse(&*resp)?;

        Ok(bare_feed.into())
    }

    pub async fn eval(&self, item: FeedItem) -> Result<Program> {
        self.runtime.process(item).await
    }

    pub async fn untrack(&self, url: Url, purge: bool) -> crate::Result<()> {
        let untrack = db::Untrack {
            url: url.as_ref(),
            purge,
        };

        untrack.run(&self.conn)?;

        Ok(())
    }

    pub async fn track(&self, url: Url, ttl: Option<u32>) -> crate::Result<()> {
        let endpoint = url.to_string();
        let feed = self.fetch_items(url).await?;

        let ttl = ttl.or(feed.meta.ttl).unwrap_or(30);
        let name = feed.meta.title.clone();

        let feed_op = db::Feed {
            name: name.as_deref(),
            url: &endpoint,
            ttl,
        };

        feed_op.run(&self.conn)?;

        let track_op = db::Track {
            url: &endpoint,
            time: Utc::now(),
        };

        track_op.run(&self.conn)?;

        // process entries

        Ok(())
    }

    pub async fn migrate(&self) -> crate::Result<()> {
        db::migrate(&self.conn)
    }

    pub async fn run(&self) -> crate::Result<()> {
        let feeds = db::GetFeed {}.run(&self.conn)?;

        let now = Utc::now();
        let watch: Vec<_> = feeds
            .into_iter()
            .map(|feed| {
                let next = feed.last_fetch + Duration::minutes(feed.ttl.into());
                let due = next < now;

                (next, due, feed)
            })
            .collect();

        dbg!(watch);

        Ok(())
    }
}
