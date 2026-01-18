use url::Url;

use crate::Feed;

#[derive(Clone)]
pub struct Fetcher {
    pub client: reqwest::Client,
}

impl Fetcher {
    pub async fn fetch_items(&self, url: Url) -> crate::Result<Feed> {
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
