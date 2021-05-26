use std::process::Command;
use std::time::Duration;

use crate::db::Database;
use cyndikator_dispatch::{Action, Event};

use log::{debug, error};
use notify_rust::Notification;
use wait_timeout::ChildExt;

pub struct Invoker<'a> {
    db: &'a mut Database,
}

impl<'a> Invoker<'a> {
    pub fn new(db: &'a mut Database) -> Invoker {
        Invoker { db }
    }

    pub fn invoke(&mut self, action: &Action, event: &Event) {
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
                    .summary(event.title.as_deref().unwrap_or("(untitled event)"))
                    .body(event.url.as_deref().unwrap_or(""))
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
                    event.description.as_deref(),
                    &event.categories,
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

fn shell_exec(cmd: &str) -> Result<(), std::io::Error> {
    let mut child = if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", cmd]).spawn()?
    } else {
        Command::new("sh").args(&["-c", cmd]).spawn()?
    };

    child.wait_timeout(Duration::from_secs(30))?;

    Ok(())
}
