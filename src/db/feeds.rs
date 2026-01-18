use rusqlite::named_params;
use tokio::sync::oneshot;

use crate::db::Operation;

#[derive(Debug)]
pub struct Insert {
    pub(crate) send: oneshot::Sender<()>,
    pub name: Option<String>,
    pub url: String,
    pub ttl: u32,
}

impl Operation for Insert {
    fn perform(self, conn: &rusqlite::Connection) -> crate::Result<()> {
        conn.execute(
            r#"
insert into feeds (name, url, ttl) 
values (:name, :url, :ttl) 
on conflict (url) 
do update set ttl = EXCLUDED.ttl"#,
            named_params! {
                ":name": self.name,
                ":url": self.url,
                ":ttl": self.ttl,

            },
        )?;

        let _ = self.send.send(());

        Ok(())
    }
}
