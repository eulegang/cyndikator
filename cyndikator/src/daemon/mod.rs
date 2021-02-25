use crate::db::Database;
use crate::ticker::Ticker;
use crate::tracker::{Trackee, Tracker};
use chrono::{DateTime, Local};
use futures::{future::join_all, select, StreamExt};
use std::time::Duration;
use url::Url;

use cyndikator_rss::{self as rss, Rss};

pub struct Daemon {
    db: Database,
    tick: usize,
}

impl Daemon {
    pub fn new(db: Database, tick: usize) -> Daemon {
        Daemon { db, tick }
    }

    pub async fn run(mut self) -> eyre::Result<()> {
        let mut tracker = Tracker::default();
        let mut ticker = Ticker::new(Duration::from_secs(self.tick as u64));

        let mut fetches = Vec::new();
        for feed in self.db.tracking()? {
            let url = match Url::parse(&feed.url) {
                Ok(url) => url,
                Err(_) => continue,
            };

            if feed.last_fetch.is_none() {
                fetches.push(self.fetch(url.clone()));
            }

            tracker.track(Trackee {
                url: url,
                last: feed.last_fetch.unwrap_or_else(|| Local::now()),
                ttl: feed.ttl.unwrap_or(60),
            })
        }

        let (rsses, errs) = sep(join_all(fetches).await);
        for (url, err) in errs {
            eprintln!("{} {}", err, url);
        }

        self.dispatch(rsses).await;

        loop {
            select! {
                now = ticker.next() => {
                    let now = if let Some(now) = now { now } else { continue };

                    let expired = tracker.expired(&now);

                    let mut fetches = Vec::new();
                    for trackee in &expired {
                        fetches.push(self.fetch(trackee.url.clone()));
                    }

                    let (rsses, errs) = sep(join_all(fetches).await);
                    for (url, err) in errs {
                        eprintln!("{} {}", err, url);
                    }

                    self.dispatch(rsses).await;

                }

            }
        }
    }

    async fn fetch(&self, url: Url) -> (Url, Result<Rss, rss::Error>) {
        let rss = Rss::fetch(&url).await;
        (url, rss)
    }

    async fn dispatch(&mut self, feeds: Vec<(Url, Rss)>) {
        for (url, feed) in feeds {
            let chan = feed.channel;

            for item in &chan.items {
                todo!()
            }
        }
    }
}

fn sep<S, T, E>(v: Vec<(S, Result<T, E>)>) -> (Vec<(S, T)>, Vec<(S, E)>) {
    let mut ts = Vec::new();
    let mut es = Vec::new();

    for (s, e) in v.into_iter() {
        match e {
            Ok(t) => ts.push((s, t)),
            Err(e) => es.push((s, e)),
        }
    }

    (ts, es)
}
