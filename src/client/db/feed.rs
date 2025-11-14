use rusqlite::{Connection, named_params};

use crate::client::db::DBOperation;

#[derive(Debug)]
pub struct Feed<'a> {
    pub name: Option<&'a str>,
    pub url: &'a str,
    pub ttl: u32,
}

impl DBOperation for Feed<'_> {
    fn run(&self, conn: &Connection) -> crate::Result<()> {
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

        Ok(())
    }
}
