use crate::db::Operation;

use super::types::Feed;
use rusqlite::Connection;
use rusqlite::fallible_iterator::FallibleIterator;
use tokio::sync::oneshot;

pub struct List(pub(crate) oneshot::Sender<Vec<Feed>>);

impl Operation for List {
    fn perform(self, conn: &Connection) -> crate::Result<()> {
        let mut prep = conn.prepare(
            "select url, ttl, last_fetch, tracking.id from feeds inner join tracking on feeds.id = tracking.feed",
        )?;

        let rows = prep.query([])?;

        let feeds: Vec<Feed> = rows
            .map(|row| {
                Ok(Feed {
                    url: row.get(0)?,
                    ttl: row.get(1)?,
                    last_fetch: row.get(2)?,
                    tracking: row.get(3)?,
                })
            })
            .collect()?;

        let _ = self.0.send(feeds);

        Ok(())
    }
}
