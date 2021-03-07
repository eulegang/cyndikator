use crate::db::Database;
use crossterm::{event::read, terminal};
use std::io::{stdout, Write};

use draw::*;
use record::Cache;

mod draw;
mod inter;
mod record;

pub struct View {
    db: Database,
}

pub enum Action {
    Delete,
    Undo,
    Open,
    Noop,
    Quit,
}

impl View {
    pub fn new(db: Database) -> View {
        View { db }
    }

    fn render(self, out: &mut impl Write) -> eyre::Result<()> {
        let (_, height) = terminal::size()?;
        let mut cache = Cache::new(self.db);
        let mut inter = inter::Inter::new(height, 0);

        loop {
            let selected = inter.offset;
            let entries = cache.window(inter.base, height as u32)?;
            Full { selected, entries }.draw(out)?;
            out.flush()?;

            let e = read()?;

            match inter.interact(&e)? {
                Action::Open => {
                    if let Some(url) = &entries
                        .get(inter.offset as usize)
                        .and_then(|e| e.url.as_ref())
                    {
                        let _ = open::that_in_background(url);
                    }
                }

                Action::Delete => {
                    cache.delete(inter.offset as usize);
                }

                Action::Undo => {
                    cache.undo();
                }

                Action::Noop => (),
                Action::Quit => break,
            };

            let total = cache.total();
            if inter.offset as u32 + inter.base >= total {
                inter.base = total.checked_sub(inter.height as u32).unwrap_or(0);

                inter.offset = total
                    .checked_sub(inter.base)
                    .unwrap_or(0)
                    .checked_sub(1)
                    .unwrap_or(0) as u16;
            }
            inter.offset = inter.offset.min(cache.total() as u16 - 1);
        }

        Ok(())
    }

    pub fn interact(self) -> eyre::Result<()> {
        let mut out = stdout();

        let raw = Raw::default();

        Clear.draw(&mut out)?;
        ShowCur(false).draw(&mut out)?;

        let res = self.render(&mut out);

        let _ = Clear.draw(&mut out);
        let _ = ShowCur(true).draw(&mut out);

        drop(raw);
        out.flush()?;

        res
    }
}

struct Raw;

impl Default for Raw {
    fn default() -> Raw {
        let _ = terminal::enable_raw_mode();

        Raw
    }
}

impl Drop for Raw {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
    }
}
