use chrono::{DateTime, Utc};
use serde::{Serialize, ser::SerializeMap};

mod lua;

#[derive(Clone, Debug, Serialize)]
pub struct Feed {
    pub meta: FeedMeta,
    pub items: Vec<FeedItem>,
}

#[derive(Clone, Debug, Serialize)]
pub struct FeedMeta {
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub authors: Vec<Person>,
    pub contributors: Vec<Person>,
    pub links: Vec<Link>,
    pub categories: Vec<Category>,
    pub ttl: Option<u32>,
    pub updated: Option<DateTime<Utc>>,
    pub published: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Serialize)]
pub struct FeedItem {
    pub id: String,
    pub title: Option<String>,
    pub authors: Vec<Person>,
    pub contributors: Vec<Person>,
    pub summary: Option<String>,
    pub content: Option<Content>,
    pub source: Option<String>,
    pub categories: Vec<Category>,
    pub links: Vec<Link>,
    pub updated: Option<DateTime<Utc>>,
    pub published: Option<DateTime<Utc>>,
    pub base: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Person {
    pub name: String,
    pub uri: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Content {
    Body(String),
    Link(Link),
}

impl Serialize for Content {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Content::Body(body) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "body")?;
                map.serialize_entry("body", body)?;
                map.end()
            }

            Content::Link(link) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "link")?;
                map.serialize_entry("link", link)?;
                map.end()
            }
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Link {
    pub href: String,
    pub rel: Option<String>,
    pub media_type: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Category {
    pub term: String,
    pub label: Option<String>,
    pub subcategories: Vec<Category>,
}

impl From<feed_rs::model::Feed> for Feed {
    fn from(value: feed_rs::model::Feed) -> Self {
        let title = value.title.map(|t| t.content);
        let description = value.description.map(|t| t.content);
        let authors = value.authors.into_iter().map(Into::into).collect();
        let contributors = value.contributors.into_iter().map(Into::into).collect();
        let items = value.entries.into_iter().map(Into::into).collect();
        let categories = value.categories.into_iter().map(Into::into).collect();
        let links = value.links.into_iter().map(Into::into).collect();

        Feed {
            meta: FeedMeta {
                id: value.id,
                title,
                description,
                authors,
                contributors,
                categories,
                links,
                ttl: value.ttl,
                updated: value.updated,
                published: value.published,
            },
            items,
        }
    }
}

impl From<feed_rs::model::Person> for Person {
    fn from(value: feed_rs::model::Person) -> Self {
        Self {
            name: value.name,
            uri: value.uri,
            email: value.email,
        }
    }
}

impl From<feed_rs::model::Entry> for FeedItem {
    fn from(value: feed_rs::model::Entry) -> Self {
        let authors = value.authors.into_iter().map(Into::into).collect();
        let contributors = value.contributors.into_iter().map(Into::into).collect();
        let categories = value.categories.into_iter().map(Into::into).collect();
        let links = value.links.into_iter().map(Into::into).collect();
        let content = value.content.and_then(|content| {
            if let Some(body) = content.body {
                Some(Content::Body(body))
            } else {
                content.src.map(|link| Content::Link(link.into()))
            }
        });

        Self {
            id: value.id,
            title: value.title.map(|t| t.content),
            authors,
            contributors,
            summary: value.summary.map(|t| t.content),
            content,
            source: value.source,
            categories,
            links,
            updated: value.updated,
            published: value.published,
            base: value.base,
        }
    }
}

impl From<feed_rs::model::Link> for Link {
    fn from(value: feed_rs::model::Link) -> Self {
        Link {
            href: value.href,
            rel: value.rel,
            media_type: value.media_type,
            title: value.title,
        }
    }
}

impl From<feed_rs::model::Category> for Category {
    fn from(value: feed_rs::model::Category) -> Self {
        let subcategories = value.subcategories.into_iter().map(Into::into).collect();

        Category {
            term: value.term,
            label: value.label,
            subcategories,
        }
    }
}
