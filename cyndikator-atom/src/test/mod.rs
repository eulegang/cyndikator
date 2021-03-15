use super::*;
use chrono::{FixedOffset, Local, TimeZone, Utc};
use std::str::FromStr;

const SPARSE: &str = include_str!("sparse.xml");
const FULL: &str = include_str!("full.xml");

#[test]
fn test_sparse() {
    let mut feed = Feed::from_str(SPARSE).unwrap();

    assert_eq!(&feed.title.content, "Example Feed");
    assert_eq!(&feed.title.ty, "text");

    assert_eq!(
        feed.links,
        vec![Link {
            rel: None,
            ty: None,
            href: "http://example.org/".to_string(),
        }]
    );

    assert_eq!(
        feed.updated,
        Timestamp {
            datetime: Utc
                .ymd(2003, 12, 13)
                .and_hms(18, 30, 02)
                .with_timezone(&Local)
        }
    );

    assert_eq!(&feed.categories, &[]);

    assert_eq!(
        feed.authors,
        vec![Person {
            name: "John Doe".to_string(),
            url: None,
            email: None,
        }]
    );

    assert_eq!(feed.subtitle, None);

    assert_eq!(&feed.id, "urn:uuid:60a76c80-d399-11d9-b93C-0003939e0af6");
    assert_eq!(feed.rights, None);

    assert_eq!(feed.contributors, vec![]);

    assert_eq!(feed.entries.len(), 1);

    let entry = feed.entries.swap_remove(0);

    assert_eq!(&entry.title.content, "Atom-Powered Robots Run Amok");
    assert_eq!(&entry.title.ty, "text");

    assert_eq!(
        entry.links,
        vec![Link {
            href: "http://example.org/2003/12/13/atom03".to_string(),
            rel: None,
            ty: None,
        }]
    );

    assert_eq!(&entry.categories, &[]);

    assert_eq!(&entry.id, "urn:uuid:1225c695-cfb8-4ebb-aaaa-80da344efa6a");

    assert_eq!(&entry.authors, &[]);
    assert_eq!(&entry.contributors, &[]);

    assert_eq!(
        entry.updated,
        Timestamp {
            datetime: Utc
                .ymd(2003, 12, 13)
                .and_hms(18, 30, 02)
                .with_timezone(&Local)
        }
    );

    assert_eq!(entry.published, None);

    assert_eq!(entry.summary.unwrap().content, "Some text.");
    assert_eq!(entry.content, None);
}

#[test]
fn test_full() {
    let mut feed = Feed::from_str(FULL).unwrap();

    assert_eq!(&feed.title.content, "dive into mark");
    assert_eq!(&feed.title.ty, "text");

    let subtitle = feed.subtitle.unwrap();

    assert_eq!(
        &subtitle.content,
        "A <em>lot</em> of effort\n    went into making this effortless"
    );
    assert_eq!(&subtitle.ty, "html");
    assert_eq!(&feed.categories, &[]);

    assert_eq!(
        feed.updated,
        Timestamp {
            datetime: Utc
                .ymd(2005, 7, 31)
                .and_hms(12, 29, 29)
                .with_timezone(&Local)
        }
    );

    assert_eq!(&feed.id, "tag:example.org,2003:3");

    assert_eq!(
        &feed.links,
        &[
            Link {
                href: "http://example.org/".to_string(),
                ty: Some("text/html".to_string()),
                rel: Some("alternate".to_string()),
            },
            Link {
                href: "http://example.org/feed.atom".to_string(),
                ty: Some("application/atom+xml".to_string()),
                rel: Some("self".to_string()),
            }
        ]
    );

    assert_eq!(
        feed.rights.as_deref(),
        Some("Copyright (c) 2003, Mark Pilgrim")
    );

    assert_eq!(&feed.authors, &[]);
    assert_eq!(&feed.contributors, &[]);

    assert_eq!(feed.entries.len(), 1);

    let entry = feed.entries.swap_remove(0);

    assert_eq!(&entry.title.content, "Atom draft-07 snapshot");
    assert_eq!(&entry.title.ty, "text");

    assert_eq!(
        &entry.links,
        &[
            Link {
                href: "http://example.org/2005/04/02/atom".to_string(),
                ty: Some("text/html".to_string()),
                rel: Some("alternate".to_string()),
            },
            Link {
                href: "http://example.org/audio/ph34r_my_podcast.mp3".to_string(),
                ty: Some("audio/mpeg".to_string()),
                rel: Some("enclosure".to_string()),
            }
        ]
    );

    assert_eq!(&entry.id, "tag:example.org,2003:3.2397");

    assert_eq!(
        feed.updated,
        Timestamp {
            datetime: Utc
                .ymd(2005, 7, 31)
                .and_hms(12, 29, 29)
                .with_timezone(&Local)
        }
    );

    assert_eq!(
        entry.published,
        Some(Timestamp {
            datetime: dbg!(FixedOffset::west(4 * 3600)
                .ymd(2003, 12, 13)
                .and_hms(8, 29, 29))
            .with_timezone(&Local)
        })
    );

    assert_eq!(
        &entry.authors,
        &[Person {
            name: "Mark Pilgrim".to_string(),
            url: Some("http://example.org/".to_string()),
            email: Some("f8dy@example.com".to_string()),
        }]
    );

    assert_eq!(
        &entry.categories,
        &[
            Category {
                term: "snapshot".to_string(),
                scheme: None,
                label: None,
            },
            Category {
                term: "proposal".to_string(),
                scheme: None,
                label: None,
            }
        ]
    );

    assert_eq!(entry.content, None);
    assert_eq!(entry.summary, None);

    assert_eq!(
        &entry.contributors,
        &[
            Person {
                name: "Sam Ruby".to_string(),
                url: None,
                email: None,
            },
            Person {
                name: "Joe Gregorio".to_string(),
                url: None,
                email: None,
            }
        ]
    );
}
