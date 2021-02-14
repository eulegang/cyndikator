use crate::{Error, Rss};
use quick_xml::de::from_str;
use std::str::FromStr;

impl FromStr for Rss {
    type Err = Error;

    fn from_str(input: &str) -> Result<Rss, Error> {
        let rss: Rss = from_str(input)?;

        Ok(rss)
    }
}

#[cfg(feature = "fetch")]
impl Rss {
    pub async fn fetch(url: url::Url) -> Result<Rss, Error> {
        match url.scheme() {
            "https" | "http" => {
                let resp = reqwest::get(url).await?.error_for_status()?;
                let text = resp.text().await?;

                Rss::from_str(&text)
            }

            a => Err(Error::InvalidScheme(a.to_string())),
        }
    }
}
