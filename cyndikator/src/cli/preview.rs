use clap::Parser;
use cyndikator_dispatch::DispatcherSource;
use eyre::ContextCompat;
use reqwest::Url;
use std::path::PathBuf;

use crate::{config::Config, fetcher::Fetcher};

use super::db_coord;

/// Preview how a dispatcher would act
#[derive(Parser)]
pub struct Preview {
    #[clap(short, long)]
    feed: Vec<String>,

    #[clap(short, long)]
    dispatch: Option<String>,

    #[clap(short, long, env = "CYNDIKATOR_CONFIG")]
    config: Option<PathBuf>,

    #[clap(short, long)]
    verbose: bool,
}

impl Preview {
    pub async fn run(self) -> eyre::Result<()> {
        let config = Config::load(self.config.as_deref())?;

        let feeds = if self.feed.is_empty() {
            let mut db = db_coord(&config.database)?.open()?;
            db.tracking()?.into_iter().map(|f| f.url).collect()
        } else {
            self.feed
        };

        let dispatch = if let Some(d) = &self.dispatch {
            if d.ends_with(".lua") {
                DispatcherSource::Lua(&PathBuf::from(d)).dispatcher()?
            } else {
                DispatcherSource::Dispatch(&PathBuf::from(d)).dispatcher()?
            }
        } else {
            let dispatch_path = config.dispatch.path.wrap_err("dispatch path not set")?;
            DispatcherSource::Dispatch(&dispatch_path).dispatcher()?
        };

        for feed in &feeds {
            println!("feed: {feed}");

            let url = match Url::parse(feed) {
                Ok(url) => url,
                Err(err) => {
                    println!("err!: {err}");
                    continue;
                }
            };

            let mut fetcher = Fetcher::new(&url);

            let events = match fetcher.events().await {
                Ok(events) => events,
                Err(err) => {
                    println!("err!: {err}");
                    continue;
                }
            };

            for event in &events {
                let actions = dispatch.dispatch(event);

                if !self.verbose && actions.is_empty() {
                    continue;
                }

                let title = event.title.as_deref().unwrap_or("untitled");
                let categories = event.categories.join(", ");

                println!("Title: {title}");
                println!("  actions: {actions:?}");
                println!("  categories: {categories}");
            }
        }

        Ok(())
    }
}
