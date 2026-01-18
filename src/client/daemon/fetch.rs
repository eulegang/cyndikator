use tokio_util::sync::CancellationToken;
use url::Url;

use crate::{db::types::Feed, fetcher::Fetcher};

pub struct FetchFeed {
    pub(crate) feed: Feed,
    pub(crate) token: CancellationToken,
    pub(crate) fetcher: Fetcher,
}

impl FetchFeed {
    pub(crate) async fn run(self) {
        let url = match Url::parse(&self.feed.url) {
            Ok(url) => url,
            Err(err) => {
                eprintln!("invalid feed url {} {}", self.feed.url, err);
                return;
            }
        };

        self.fetcher.fetch_items(url).await;
    }
}
