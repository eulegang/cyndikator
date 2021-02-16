use rusqlite::{Connection, OpenFlags};

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
}
