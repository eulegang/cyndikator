use url::Url;

use crate::{feed::Feed, Result};

pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new() -> Client {
        let client = reqwest::Client::new();

        Self { client }
    }

    pub async fn fetch_items(&self, url: Url) -> Result<Feed> {
        let resp = self
            .client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        let bare_feed = feed_rs::parser::parse(&*resp)?;

        Ok(bare_feed.into())
    }
}
