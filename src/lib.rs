mod client;
mod feed;

pub use client::Client;
pub use feed::{Feed, FeedItem};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to fetch: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("unable to parse feed: {0}")]
    FeedParse(#[from] feed_rs::parser::ParseFeedError),
}

pub type Result<T> = std::result::Result<T, Error>;
