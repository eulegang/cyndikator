use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Rss {
    pub channel: Channel,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Channel {
    pub title: String,
    pub description: String,
    pub language: Option<String>,
    pub copyright: Option<String>,
    pub managingEditor: Option<String>,
    pub webMaster: Option<String>,

    #[serde(rename = "item")]
    pub items: Vec<Item>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Item {
    pub title: Option<String>,
    pub link: Option<String>,
    pub author: Option<String>,
    pub category: Vec<String>,
    pub comments: Option<String>,
    pub guid: Option<Guid>,
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
