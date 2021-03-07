use chrono::{DateTime, Local};
use cyndikator_rss::Rss;
use rusqlite::{config::DbConfig, params, Connection, OpenFlags};
use url::Url;

use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

mod migrate {
    use refinery::embed_migrations;
    embed_migrations!();
}

pub struct Database {
    conn: Connection,
}

#[derive(Debug)]
pub struct Feed {
    pub title: String,
    pub url: String,
    pub ttl: Option<u32>,
    pub last_fetch: Option<DateTime<Local>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Entry {
    pub id: u32,
    pub url: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub categories: Vec<String>,
    pub feed: Option<String>,
    pub feed_url: String,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("sqlite failure {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("failure migrating {0}")]
    Migration(#[from] refinery::Error),
}

impl Database {
    pub fn open(path: impl AsRef<Path>) -> Result<Database, Error> {
        let path = path.as_ref();
        let conn = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_WRITE)?;
        conn.set_db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_FKEY, true)?;

        Ok(Database { conn })
    }

    pub fn create(path: impl AsRef<Path>) -> Result<Database, Error> {
        let path = path.as_ref();

        let conn = Connection::open(path)?;

        Ok(Database { conn })
    }

    pub fn migrate(&mut self) -> Result<(), Error> {
        migrate::migrations::runner().run(&mut self.conn)?;

        Ok(())
    }

    pub fn default_path() -> PathBuf {
        let mut dir = dirs::home_dir().expect("can not find home dir");

        dir.push(".cyndikator");

        if !dir.exists() {
            create_dir_all(&dir).expect("creating cyndikator directory");
        }

        dir.push("cynd.db3");

        dir
    }

    pub fn track(&mut self, url: &Url, rss: &Rss, ttl: Option<u32>) -> Result<(), Error> {
        if let Some(ttl) = ttl {
            self.conn.execute(
                "insert into feeds 
            (url, title, ttl) values 
            (?1, ?2, ?3)",
                params![url.as_ref(), &rss.channel.title, ttl],
            )?;
        } else {
            self.conn.execute(
                "insert into feeds 
            (url, title) values 
            (?1, ?2)",
                params![url.as_ref(), &rss.channel.title],
            )?;
        }

        Ok(())
    }

    pub fn tracking(&mut self) -> Result<Vec<Feed>, Error> {
        let mut stmt = self
            .conn
            .prepare("select title, url, ttl, last_fetch from feeds")?;

        let iter = stmt.query_map(params![], |row| {
            Ok(Feed {
                title: row.get(0)?,
                url: row.get(1)?,
                ttl: row.get(2)?,
                last_fetch: row.get(3)?,
            })
        })?;

        let mut buf = Vec::new();

        for row in iter {
            buf.push(row?);
        }

        Ok(buf)
    }

    pub fn untrack(&mut self, url: &str) -> Result<bool, Error> {
        let affected = self
            .conn
            .execute("delete from feeds where url = ?1", params![url])?;

        Ok(affected > 0)
    }

    pub fn record(
        &mut self,
        feed_url: &str,
        name: Option<&str>,
        url: Option<&str>,
        description: Option<&str>,
        categories: &[String],
    ) -> Result<(), Error> {
        self.conn.execute(
            "
                insert into items (title, url, feed_id, description, categories) 
                select ?1 title, ?2 url, id feed_id, ?4 description, ?5 categories
                from feeds where url = ?3
            "
            .trim(),
            params![name, url, feed_url, description, categories.join("\x1e")],
        )?;

        Ok(())
    }

    pub fn mark_clean(&mut self, url: &str) -> Result<bool, Error> {
        let size = self.conn.execute(
            "update feeds set last_fetch = datetime('now') where url = ?1",
            params![url],
        )?;

        Ok(size != 0)
    }

    pub fn last_fetch(&mut self, url: &str) -> Result<DateTime<Local>, Error> {
        let timestamp = self.conn.query_row(
            "select last_fetch from feeds where url = ?1",
            params![url],
            |row| row.get(0),
        )?;

        Ok(timestamp)
    }

    pub fn count_records(&self) -> Result<u32, Error> {
        let cnt = self
            .conn
            .query_row("select count(id) from items", params![], |row| {
                let cnt: u32 = row.get(0)?;
                Ok(cnt)
            })?;

        Ok(cnt)
    }

    pub fn records(&self, offset: u32, win: u32) -> Result<Vec<Entry>, Error> {
        let mut stmt = self.conn.prepare(
            "select 
               items.id id, 
               items.url url, 
               items.title title, 
               items.description description,
               items.categories categories,
               feeds.title feed_title,
               feeds.url feed_url
             from items inner join feeds 
               on items.feed_id = feeds.id
             order by items.id desc
               limit ?1 offset ?2",
        )?;

        let iter = stmt.query_map(params![win, offset], |row| {
            Ok(Entry {
                id: row.get("id")?,
                url: row.get("url")?,
                title: row.get("title")?,
                description: row.get("description")?,
                categories: row
                    .get::<&str, Option<String>>("categories")?
                    .unwrap_or_default()
                    .split('\x1e')
                    .map(ToString::to_string)
                    .collect(),

                feed: row.get("feed_title")?,
                feed_url: row.get("feed_url")?,
            })
        })?;

        let mut buf = Vec::with_capacity(win as usize);

        for row in iter {
            buf.push(row?);
        }

        Ok(buf)
    }

    pub fn delete_record(&self, id: u32) -> Result<bool, Error> {
        let affected = self
            .conn
            .execute("delete from items where id = ?1", params![id])?;
        Ok(affected > 0)
    }

    pub fn insert_record(&self, entry: &Entry) -> Result<bool, Error> {
        let affected = self.conn.execute(
            "
        insert into items (id, url, title, description, categories, feed_id) 
        select ?1 id, ?2 url, ?3 title, ?4 description, ?5 categories, feeds.id
        from feeds where url = ?6
        ",
            params![
                entry.id,
                entry.url,
                entry.title,
                entry.description,
                entry.categories.join("\x1e"),
                entry.feed_url
            ],
        )?;

        Ok(affected > 0)
    }
}
