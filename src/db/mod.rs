use crate::{Error, Result};
use chrono::{DateTime, Utc};
use rusqlite::Connection;
use tokio::sync::oneshot;

pub mod types;

mod feeds;
mod list;
mod tracking;

const BASE_SCHEMA: &str = include_str!("schema.sql");

enum Request {
    List(list::List),
    Insert(feeds::Insert),
    Track(tracking::Track),
    Untrack(tracking::Untrack),
}

trait Operation {
    fn perform(self, conn: &Connection) -> Result<()>;
}

pub struct Conn {
    send: std::sync::mpsc::Sender<Request>,
}

impl Conn {
    pub fn new(conn: Connection, migrate: bool) -> crate::Result<Self> {
        if migrate {
            conn.execute_batch(BASE_SCHEMA)?;
        }

        let (send, recv) = std::sync::mpsc::channel();
        std::thread::spawn(|| Conn::main(conn, recv));

        Ok(Self { send })
    }

    pub async fn list(&self) -> crate::Result<Vec<types::Feed>> {
        let (send, recv) = oneshot::channel();
        self.send
            .send(Request::List(list::List(send)))
            .map_err(|_| Error::RuntimeQuitSend)?;

        Ok(recv.await?)
    }

    pub async fn insert(&self, name: Option<String>, url: String, ttl: u32) -> crate::Result<()> {
        let (send, recv) = oneshot::channel();
        self.send
            .send(Request::Insert(feeds::Insert {
                send,
                name,
                url,
                ttl,
            }))
            .map_err(|_| Error::RuntimeQuitSend)?;

        Ok(recv.await?)
    }

    pub async fn track(&self, url: String, time: DateTime<Utc>) -> crate::Result<()> {
        let (send, recv) = oneshot::channel();

        self.send
            .send(Request::Track(tracking::Track { send, url, time }))
            .map_err(|_| Error::RuntimeQuitSend)?;

        Ok(recv.await?)
    }

    pub async fn untrack(&self, url: String, purge: bool) -> crate::Result<()> {
        let (send, recv) = oneshot::channel();

        self.send
            .send(Request::Untrack(tracking::Untrack { send, url, purge }))
            .map_err(|_| Error::RuntimeQuitSend)?;

        Ok(recv.await?)
    }

    fn main(conn: Connection, recv: std::sync::mpsc::Receiver<Request>) {
        while let Ok(req) = recv.recv() {
            let _ = req.perform(&conn);
        }
    }
}

impl Operation for Request {
    fn perform(self, conn: &Connection) -> Result<()> {
        match self {
            Request::List(list) => list.perform(conn),
            Request::Insert(insert) => insert.perform(conn),
            Request::Track(track) => track.perform(conn),
            Request::Untrack(untrack) => untrack.perform(conn),
        }
    }
}
