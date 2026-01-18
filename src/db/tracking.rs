use chrono::{DateTime, Utc};
use rusqlite::named_params;
use tokio::sync::oneshot;

pub struct Track {
    pub(crate) send: oneshot::Sender<()>,
    pub(crate) url: String,
    pub(crate) time: DateTime<Utc>,
}

pub struct Untrack {
    pub(crate) send: oneshot::Sender<()>,
    pub(crate) url: String,
    pub(crate) purge: bool,
}

impl super::Operation for Track {
    fn perform(self, conn: &rusqlite::Connection) -> crate::Result<()> {
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

        let _ = self.send.send(());

        Ok(())
    }
}

impl super::Operation for Untrack {
    fn perform(self, conn: &rusqlite::Connection) -> crate::Result<()> {
        conn.execute(
            r#"
            delete from tracking where feed in 
            (select id from feeds where feeds.url = :url)
            "#,
            named_params! {
                ":url": self.url,
            },
        )?;

        if self.purge {
            conn.execute(
                "delete from feeds where url = :url",
                named_params! {
                    ":url": self.url,
                },
            )?;
        }

        let _ = self.send.send(());

        Ok(())
    }
}
