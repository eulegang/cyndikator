use chrono::{DateTime, Utc};
use rusqlite::{Connection, named_params};

use crate::client::db::DBOperation;

#[derive(Debug)]
pub struct Track<'a> {
    pub url: &'a str,
    pub time: DateTime<Utc>,
}

impl DBOperation for Track<'_> {
    type T = ();

    fn run(&self, conn: &Connection) -> crate::Result<()> {
        conn.execute(
            r#"
            insert into tracking (feed, last_fetch)
            select id feed, :time last_fetch
            from feeds where feeds.url = :url
            on conflict(feed)
            do update set last_fetch = :time
            "#,
            named_params! {
                ":url": self.url,
                ":time": self.time,
            },
        )?;

        Ok(())
    }
}
