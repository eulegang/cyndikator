#![allow(clippy::new_without_default)]

mod client;
mod db;
mod feed;
mod fetcher;
mod interp;
mod runtime;

pub use client::Client;
pub use feed::{Feed, FeedItem};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to fetch: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("unable to parse feed: {0}")]
    FeedParse(#[from] feed_rs::parser::ParseFeedError),

    #[error("failed to transact db: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("Shutdown runtime")]
    RuntimeShutdown,

    #[error("invalid setup")]
    InvalidSetup,

    #[error("runtime quit (recv)")]
    RuntimeQuitRecv(#[from] tokio::sync::oneshot::error::RecvError),

    #[error("runtime quit (send)")]
    RuntimeQuitSend,
}

pub type Result<T> = std::result::Result<T, Error>;
