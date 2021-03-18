use super::FetcherFormat;
use cyndikator_atom::Feed;
use cyndikator_dispatch::Event;
use std::str::FromStr;
use url::Url;

pub struct Format;

impl FetcherFormat for Format {
    fn title(&self, content: &str) -> eyre::Result<String> {
        let feed = Feed::from_str(content)?;
        Ok(feed.title.content)
    }

    fn events(&self, url: &Url, content: &str) -> eyre::Result<Vec<Event>> {
        let feed = Feed::from_str(content)?;

        let mut results = Vec::with_capacity(feed.entries.len());

        for entry in feed.entries {
            let feed_url = url.to_string();
            let feed_title = Some(feed.title.content.clone());
            let feed_categories = feed.categories.iter().map(|c| c.term.clone()).collect();

            let url = entry
                .links
                .iter()
                .find(|l| l.ty.as_deref() == Some("text/html"))
                .or_else(|| entry.links.iter().find(|l| l.ty == None))
                .map(|l| l.href.clone());
            let title = Some(entry.title.content);
            let description = entry.summary.map(|t| t.content.clone());
            let categories = entry.categories.iter().map(|c| c.term.clone()).collect();
            let date = Some(entry.updated.into());

            results.push(Event {
                url,
                title,
                categories,
                description,

                feed_url,
                feed_title,
                feed_categories,

                date,
            });
        }

        Ok(results)
    }
}
