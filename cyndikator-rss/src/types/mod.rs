use chrono::{DateTime, Local};
use serde::Deserialize;

mod deser;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Rss {
    pub channel: Channel,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Channel {
    pub title: String,
    pub description: HtmlEncoded,
    pub language: Option<String>,
    pub copyright: Option<String>,

    #[serde(rename = "managingEditor")]
    pub managing_editor: Option<String>,
    #[serde(rename = "webMaster")]
    pub web_master: Option<String>,

    #[serde(rename = "pubDate")]
    pub pub_date: Option<Timestamp>,
    #[serde(rename = "lastBuildDate")]
    pub last_build_date: Option<Timestamp>,

    pub category: Option<Vec<String>>,
    pub generator: Option<String>,
    pub ttl: Option<usize>,

    #[serde(rename = "item")]
    pub items: Vec<Item>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Item {
    pub title: Option<String>,
    pub link: Option<String>,
    pub author: Option<String>,
    pub category: Option<Vec<String>>,
    pub comments: Option<String>,
    pub guid: Option<Guid>,
    pub description: Option<HtmlEncoded>,

    #[serde(rename = "pubDate")]
    pub pub_date: Option<Timestamp>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Guid {
    #[serde(default = "Guid::default_perma", rename = "isPermalink")]
    pub is_permalink: bool,

    #[serde(rename = "$value")]
    pub link: String,
}

impl Guid {
    fn default_perma() -> bool {
        true
    }
}

#[derive(PartialEq, Debug)]
pub struct Timestamp {
    pub(crate) datetime: DateTime<Local>,
}

impl std::ops::Deref for Timestamp {
    type Target = DateTime<Local>;

    fn deref(&self) -> &DateTime<Local> {
        &self.datetime
    }
}

#[derive(PartialEq, Debug)]
pub struct HtmlEncoded(pub(crate) String);

impl std::ops::Deref for HtmlEncoded {
    type Target = str;

    fn deref(&self) -> &str {
        self.0.deref()
    }
}

impl Into<String> for HtmlEncoded {
    fn into(self) -> String {
        self.0
    }
}
