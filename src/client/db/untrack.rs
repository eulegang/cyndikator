use rusqlite::named_params;

use crate::client::db::DBOperation;

pub struct Untrack<'a> {
    pub url: &'a str,
    pub purge: bool,
}

impl DBOperation for Untrack<'_> {
    fn run(&self, conn: &rusqlite::Connection) -> crate::Result<()> {
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

        Ok(())
    }
}
