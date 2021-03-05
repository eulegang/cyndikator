use crate::db::Database;
use crate::ticker::Ticker;
use crate::tracker::{Trackee, Tracker};
use chrono::Local;
use futures::{future::join_all, select, StreamExt};
use log::{debug, error, info, trace};
use notify_rust::Notification;
use std::process::Command;
use std::time::Duration;
use url::Url;
use wait_timeout::ChildExt;

use cyndikator_dispatch::{Action, Dispatch, Event};
use cyndikator_rss::{self as rss, Rss};

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

        let (rsses, errs) = sep(join_all(fetches).await);
        for (url, err) in errs {
            error!("error while tracking feed '{}': {}", url, err);
        }

        self.dispatch(rsses).await;

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
            let last_fetch = self.db.last_fetch(url.as_str()).ok();

            let chan = feed.channel;

            for item in &chan.items {
                debug!("{:?} {:?}", &last_fetch, &item.pub_date);
                match (&last_fetch, &item.pub_date) {
                    (Some(lf), Some(pd)) if lf >= pd => {
                        debug!(
                            "skipping {} {}",
                            &chan.title,
                            item.title.as_deref().unwrap_or("''")
                        );

                        continue;
                    }

                    _ => (),
                };

                let event = Event {
                    url: item.link.clone(),
                    title: item.title.clone(),
                    categories: item.category.clone().unwrap_or_default(),

                    feed_url: url.to_string(),
                    feed_title: Some(chan.title.clone()),
                    feed_categories: chan.category.clone().unwrap_or_default(),
                };

                info!(
                    "dispatching event {} {} {} {}",
                    event.feed_title.as_deref().unwrap_or("''"),
                    event.title.as_deref().unwrap_or("''"),
                    event.feed_url,
                    event.url.as_deref().unwrap_or("''"),
                );

                let actions = self.dispatch.dispatch(&event);

                for action in &actions {
                    self.perform(action, &event).await;
                }
            }

            if let Err(err) = self.db.mark_clean(url.as_str()) {
                error!("failed to mark {} as clean: {}", &url, err);
            }
        }
    }

    async fn perform(&mut self, action: &Action, event: &Event) {
        match action {
            Action::Notify => {
                debug!(
                    "dispatching event {} {} {} {}",
                    event.feed_title.as_deref().unwrap_or("''"),
                    event.title.as_deref().unwrap_or("''"),
                    event.feed_url,
                    event.url.as_deref().unwrap_or("''"),
                );

                let res = Notification::new()
                    .summary(event.title.as_deref().unwrap_or_else(|| "(untitled event)"))
                    .body(event.url.as_deref().unwrap_or_else(|| ""))
                    .show();

                if let Err(err) = res {
                    error!("error notifing {}", err)
                }
            }

            Action::Record => {
                debug!(
                    "recording event {} {} {} {}",
                    event.feed_title.as_deref().unwrap_or("''"),
                    event.title.as_deref().unwrap_or("''"),
                    event.feed_url,
                    event.url.as_deref().unwrap_or("''"),
                );

                let res = self.db.record(
                    &event.feed_url,
                    event.title.as_deref(),
                    event.url.as_deref(),
                );

                if let Err(err) = res {
                    error!("error recording {}", err)
                }
            }

            Action::Exec(cmd) => {
                debug!(
                    "execing event {} {} {} {} `{}`",
                    event.feed_title.as_deref().unwrap_or("''"),
                    event.title.as_deref().unwrap_or("''"),
                    event.feed_url,
                    event.url.as_deref().unwrap_or("''"),
                    cmd,
                );

                let res = shell_exec(cmd);

                if let Err(err) = res {
                    error!("error execing {}", err)
                }
            }
        };
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

fn shell_exec(cmd: &str) -> Result<(), std::io::Error> {
    let mut child = Command::new("sh").arg("-c").arg(cmd).spawn()?;

    child.wait_timeout(Duration::from_secs(30))?;

    Ok(())
}
