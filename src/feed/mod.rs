use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Feed {
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
    pub items: Vec<FeedItem>,
}

#[derive(Debug)]
pub struct FeedItem {
    pub id: String,
}

#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub uri: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug)]
pub struct Link {
    pub href: String,
    pub rel: Option<String>,
    pub media_type: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug)]
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
            id: value.id,
            title,
            description,
            items,
            authors,
            contributors,
            categories,
            links,
            ttl: value.ttl,
            updated: value.updated,
            published: value.published,
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
        let id = value.id;

        Self { id }
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
