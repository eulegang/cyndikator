use chrono::{DateTime, Local};
use serde::Deserialize;

mod deser;
#[derive(Deserialize, Debug, PartialEq)]
pub struct Feed {
    pub title: Text,
    pub subtitle: Option<Text>,
    pub updated: Timestamp,
    pub id: String,

    #[serde(rename = "link", default)]
    pub links: Vec<Link>,
    pub rights: Option<String>,

    #[serde(rename = "author", default)]
    pub authors: Vec<Person>,
    #[serde(rename = "contributor", default)]
    pub contributors: Vec<Person>,

    #[serde(rename = "category", default)]
    pub categories: Vec<Category>,

    #[serde(rename = "entry", default)]
    pub entries: Vec<Entry>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Entry {
    pub title: Text,
    #[serde(rename = "link", default)]
    pub links: Vec<Link>,
    pub id: String,
    #[serde(rename = "author", default)]
    pub authors: Vec<Person>,
    #[serde(rename = "contributor", default)]
    pub contributors: Vec<Person>,
    #[serde(rename = "category", default)]
    pub categories: Vec<Category>,
    pub updated: Timestamp,
    pub published: Option<Timestamp>,
    pub summary: Option<Text>,
    pub content: Option<Text>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Text {
    #[serde(rename = "$value")]
    pub content: String,
    #[serde(rename = "type", default = "Text::default_type")]
    pub ty: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Person {
    pub name: String,
    #[serde(rename = "uri")]
    pub url: Option<String>,
    pub email: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Link {
    pub rel: Option<String>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub href: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Category {
    pub term: String,
    pub scheme: Option<String>,
    pub label: Option<String>,
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

impl Into<DateTime<Local>> for Timestamp {
    fn into(self) -> DateTime<Local> {
        self.datetime
    }
}

impl Text {
    fn default_type() -> String {
        "text".to_string()
    }
}
