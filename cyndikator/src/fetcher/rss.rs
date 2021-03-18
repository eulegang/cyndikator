use super::FetcherFormat;
use cyndikator_dispatch::Event;
use cyndikator_rss::Rss;
use std::str::FromStr;
use url::Url;

pub struct Format;

impl FetcherFormat for Format {
    fn title(&self, content: &str) -> eyre::Result<String> {
        let feed = Rss::from_str(content)?;

        Ok(feed.channel.title)
    }

    fn events(&self, url: &Url, content: &str) -> eyre::Result<Vec<Event>> {
        let feed = Rss::from_str(content)?;

        let chan = feed.channel;
        let mut results = Vec::with_capacity(chan.items.len());

        for item in chan.items {
            let description = item.description.map(Into::into);

            let event = Event {
                url: item.link.clone(),
                title: item.title.clone(),
                categories: item.category.clone().unwrap_or_default(),
                description,

                feed_url: url.to_string(),
                feed_title: Some(chan.title.clone()),
                feed_categories: chan.category.clone().unwrap_or_default(),

                date: item.pub_date.map(Into::into),
            };

            results.push(event);
        }

        Ok(results)
    }
}
