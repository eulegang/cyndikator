use std::path::PathBuf;

use url::Url;

use crate::{
    FeedItem, Result,
    feed::Feed,
    runtime::{Program, Runtime},
};

pub struct Client {
    client: reqwest::Client,
    runtime: Runtime,
}

impl Client {
    pub fn new(path: Option<PathBuf>) -> Result<Client> {
        let client = reqwest::Client::new();

        let path = path
            .or_else(|| {
                let mut dir = dirs::config_dir()?;
                dir.push("cyndikator");
                dir.push("init.lua");
                Some(dir)
            })
            .ok_or(crate::Error::InvalidSetup)?;

        let runtime = Runtime::new(path);

        Ok(Self { client, runtime })
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

    pub async fn eval(&self, item: FeedItem) -> Result<Program> {
        self.runtime.process(item).await
    }
}
