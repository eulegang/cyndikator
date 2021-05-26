use crate::db::Database;
use crate::fetcher::Fetcher;
use crate::tracker::{Trackee, Tracker};
use ticker::Ticker;

use chrono::Local;
use futures::{future::join_all, select, StreamExt};
use log::{debug, error, info, trace};
use std::time::Duration;
use url::Url;

use cyndikator_dispatch::{Dispatch, Event};
use perform::Invoker;

mod perform;
mod ticker;

pub struct Daemon {
    db: Database,
    dispatch: Dispatch,
    tick: usize,
}

impl Daemon {
    pub fn new(db: Database, dispatch: Dispatch, tick: usize) -> Daemon {
        Daemon { db, dispatch, tick }
    }

    pub async fn run(mut self) -> eyre::Result<()> {
        let mut tracker = Tracker::default();
        let mut ticker = Ticker::new(Duration::from_secs(self.tick as u64));

        let mut fetches = Vec::new();
        for feed in self.db.tracking()? {
            info!("fetching {} ...", &feed.url);
            let url = match Url::parse(&feed.url) {
                Ok(url) => url,
                Err(_) => continue,
            };

            if feed.last_fetch.is_none() {
                fetches.push(self.fetch(url.clone()));
            }

            info!("tracking {}", &feed.title);

            tracker.track(Trackee {
                url: url,
                last: feed.last_fetch.unwrap_or_else(|| Local::now()),
                ttl: feed.ttl.unwrap_or(60),
            })
        }

        let (events, errs) = sep(join_all(fetches).await);
        for (url, err) in errs {
            error!("error while tracking feed '{}': {}", url, err);
        }

        self.dispatch(events).await;

        loop {
            select! {
                now = ticker.next() => {
                    trace!("tick");
                    let now = if let Some(now) = now { now } else { continue };
                    debug!("checking expired {}", now);

                    let expired = tracker.expired(&now);

                    let mut fetches = Vec::new();
                    for mut trackee in expired {
                        info!("fetching {}", trackee.url);
                        fetches.push(self.fetch(trackee.url.clone()));

                        trackee.fetched(&now);
                        tracker.track(trackee);
                    }

                    let (events, errs) = sep(join_all(fetches).await);
                    for (url, err) in errs {
                        error!("{} {}", err, url);
                    }


                    self.dispatch(events).await;
                }

            }
        }
    }

    async fn fetch(&self, url: Url) -> (Url, eyre::Result<Vec<Event>>) {
        let mut fetcher = Fetcher::new(&url);
        let events = fetcher.events().await;
        (url, events)
    }

    async fn dispatch(&mut self, feeds: Vec<(Url, Vec<Event>)>) {
        for (url, events) in feeds {
            let last_fetch = self.db.last_fetch(url.as_str()).ok();

            for event in events {
                match (&last_fetch, &event.date) {
                    (Some(lf), Some(pd)) if lf >= pd => {
                        debug!(
                            "skipping {} {}",
                            &event.feed_title.as_deref().unwrap_or("''"),
                            event.title.as_deref().unwrap_or("''")
                        );

                        continue;
                    }

                    _ => (),
                };

                debug!(
                    "dispatching event {} {} {} {}",
                    event.feed_title.as_deref().unwrap_or("''"),
                    event.title.as_deref().unwrap_or("''"),
                    event.feed_url,
                    event.url.as_deref().unwrap_or("''"),
                );

                let actions = self.dispatch.dispatch(&event);

                let mut invoker = Invoker::new(&mut self.db);
                for action in &actions {
                    invoker.invoke(action, &event);
                }
            }

            if let Err(err) = self.db.mark_clean(url.as_str()) {
                error!("failed to mark {} as clean: {}", &url, err);
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
