use chrono::{DateTime, Local};
use cyndikator_rss::Rss;
use rusqlite::{params, Connection, OpenFlags};
use url::Url;

use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

mod migrate {
    use refinery::embed_migrations;
    embed_migrations!();
}

pub struct Database {
    conn: Connection,
    pid_path: PathBuf,
}

#[derive(Debug)]
pub struct Feed {
    pub title: String,
    pub url: String,
    pub ttl: Option<u32>,
    pub last_fetch: Option<DateTime<Local>>,
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

        let mut pid_path: PathBuf = path.into();
        pid_path.pop();
        pid_path.push("pid");

        Ok(Database { conn, pid_path })
    }

    pub fn create(path: impl AsRef<Path>) -> Result<Database, Error> {
        let path = path.as_ref();

        let conn = Connection::open(path)?;
        let mut pid_path: PathBuf = path.into();
        pid_path.pop();
        pid_path.push("pid");

        Ok(Database { conn, pid_path })
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
}
