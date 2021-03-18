use eyre::bail;
use url::Url;

mod atom;
mod rss;

use cyndikator_dispatch::Event;

pub struct Fetcher {
    url: Url,
    content: Option<String>,
}

impl Fetcher {
    pub fn new(url: &Url) -> Fetcher {
        let content = None;
        let url = url.clone();

        Fetcher { url, content }
    }

    pub async fn title(&mut self) -> eyre::Result<String> {
        self.fill_cache().await?;
        let text = self.content.as_ref().unwrap();

        atom::Format.or(rss::Format).title(text)
    }

    pub async fn events(&mut self) -> eyre::Result<Vec<Event>> {
        self.fill_cache().await?;
        let text = self.content.as_ref().unwrap();

        atom::Format.or(rss::Format).events(&self.url, text)
    }

    async fn fill_cache<'a>(&'a mut self) -> eyre::Result<()> {
        if self.content.is_none() {
            let url = self.url.clone();
            match url.scheme() {
                "https" | "http" => {
                    let resp = reqwest::get(url).await?.error_for_status()?;
                    let text = resp.text().await?;
                    self.content = Some(text);
                }

                a => bail!("invalid scheme {}", a),
            }
        }

        Ok(())
    }
}

trait FetcherFormat {
    fn title(&self, content: &str) -> eyre::Result<String>;
    fn events(&self, url: &Url, content: &str) -> eyre::Result<Vec<Event>>;

    fn or<F: FetcherFormat>(self, next: F) -> Composite<Self, F>
    where
        Self: Sized,
    {
        let first = self;
        let second = next;
        Composite { first, second }
    }
}

struct Composite<A: FetcherFormat + Sized, B: FetcherFormat + Sized> {
    first: A,
    second: B,
}

impl<A, B> FetcherFormat for Composite<A, B>
where
    A: FetcherFormat + Sized,
    B: FetcherFormat + Sized,
{
    fn title(&self, content: &str) -> eyre::Result<String> {
        self.first
            .title(content)
            .or_else(|_| self.second.title(content))
    }

    fn events(&self, url: &Url, content: &str) -> eyre::Result<Vec<Event>> {
        self.first
            .events(url, content)
            .or_else(|_| self.second.events(url, content))
    }
}
