#![allow(clippy::new_without_default)]

mod client;
mod feed;
mod runtime;

pub use client::Client;
pub use feed::{Feed, FeedItem};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to fetch: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("unable to parse feed: {0}")]
    FeedParse(#[from] feed_rs::parser::ParseFeedError),

    #[error("Shutdown runtime")]
    RuntimeShutdown,

    #[error("invalid setup")]
    InvalidSetup,
}

pub type Result<T> = std::result::Result<T, Error>;
