use rusqlite::Connection;

mod feed;
mod track;
mod untrack;

pub use feed::Feed;
pub use track::Track;
pub use untrack::Untrack;

const BASE_SCHEMA: &str = include_str!("schema.sql");

pub fn migrate(conn: &Connection) -> crate::Result<()> {
    conn.execute_batch(BASE_SCHEMA)?;
    Ok(())
}

pub trait DBOperation {
    fn run(&self, conn: &Connection) -> crate::Result<()>;
}
