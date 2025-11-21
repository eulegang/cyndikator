use crate::client::db::DBOperation;
use chrono::{DateTime, Utc};
use rusqlite::fallible_iterator::FallibleIterator;

pub struct GetFeed {}

#[derive(Debug)]
pub struct Feed {
    pub url: String,
    pub ttl: u32,
    pub last_fetch: DateTime<Utc>,
    pub tracking: u32,
}

impl DBOperation for GetFeed {
    type T = Vec<Feed>;

    fn run(&self, conn: &rusqlite::Connection) -> crate::Result<Self::T> {
        let mut prep = conn.prepare(
            "select url, ttl, last_fetch, tracking.id from feeds inner join tracking on feeds.id = tracking.feed",
        )?;

        let rows = prep.query([])?;

        let feeds = rows
            .map(|row| {
                Ok(Feed {
                    url: row.get(0)?,
                    ttl: row.get(1)?,
                    last_fetch: row.get(2)?,
                    tracking: row.get(3)?,
                })
            })
            .collect()?;

        Ok(feeds)
    }
}
