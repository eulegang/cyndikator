use eyre::bail;
use feed_rs::{model::Feed, parser::parse_with_uri};
use std::fs::read_to_string;
use url::Url;

use cyndikator_dispatch::Event;

pub struct Fetcher {
    url: Url,
    feed: Option<Feed>,
}

impl Fetcher {
    pub fn new(url: &Url) -> Fetcher {
        let feed = None;
        let url = url.clone();

        Fetcher { url, feed }
    }

    pub async fn title(&mut self) -> eyre::Result<String> {
        self.fill_cache().await?;

        Ok(self
            .feed
            .as_ref()
            .unwrap()
            .title
            .as_ref()
            .map(|e| e.content.as_str())
            .unwrap_or("Untitled")
            .to_string())
    }

    pub async fn events(&mut self) -> eyre::Result<Vec<Event>> {
        self.fill_cache().await?;

        let feed = self.feed.as_ref().unwrap();

        let mut events = Vec::new();
        for entry in &feed.entries {
            let feed_url = self.url.as_str().to_string();
            let feed_title = feed.title.as_ref().map(|t| t.content.clone());
            let feed_categories = feed.categories.iter().map(|c| c.term.clone()).collect();

            let title = entry.title.as_ref().map(|t| t.content.clone());
            let url = entry.links.first().map(|l| l.href.clone());
            let categories = entry.categories.iter().map(|c| c.term.clone()).collect();
            let date = entry.published.or(entry.updated).map(|d| d.into());

            let description = entry
                .content
                .as_ref()
                .and_then(|c| c.body.as_ref())
                .map(|s| s.to_string());

            events.push(Event {
                url,
                title,
                categories,
                description,

                feed_url,
                feed_title,
                feed_categories,
                date,
            })
        }

        Ok(events)
    }

    async fn fill_cache(&mut self) -> eyre::Result<()> {
        if self.feed.is_none() {
            let url = self.url.clone();
            let text = match url.scheme() {
                "https" | "http" => {
                    let resp = reqwest::get(url).await?.error_for_status()?;
                    resp.text().await?
                }

                "file" => read_to_string(url.path())?,

                a => bail!("invalid scheme {}", a),
            };

            self.feed = Some(parse_with_uri(text.as_bytes(), Some(self.url.as_str()))?);
        }

        Ok(())
    }
}

#[cfg(test)]
#[test]
#[ignore]
fn show_lobster_feed() {
    let url = Url::parse("https://lobste.rs/rss").unwrap();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut fetcher = Fetcher::new(&url);

    rt.block_on(async {
        dbg!(fetcher.events().await.unwrap());
    });

    panic!("just showing events");
}

#[cfg(test)]
#[test]
#[ignore]
fn show_lime_feed() {
    let url = Url::parse("https://fasterthanli.me/index.xml").unwrap();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut fetcher = Fetcher::new(&url);

    rt.block_on(async {
        dbg!(fetcher.events().await.unwrap());
    });

    panic!("just showing events");
}
