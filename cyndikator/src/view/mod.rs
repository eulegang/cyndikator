use crate::db::Database;
use crossterm::{event::read, terminal};
use std::io::{stdout, Write};
use std::panic;

use draw::*;
use raw::Raw;
use record::Cache;
use stat::State;

mod draw;
mod inter;
mod raw;
mod record;
mod stat;

pub struct View {
    db: Database,
}

pub trait Inducable<T> {
    fn induce(&mut self, elem: &T);
}

pub enum ScrollUnit {
    Line,
    Half,
    Page,
}

pub enum Position {
    Abs(u32),
    Last,
}

pub enum Action {
    RelUp(u16, ScrollUnit),
    RelDown(u16, ScrollUnit),
    Goto(Position),
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
        let mut state = State::new(height);

        loop {
            let selected = state.offset();
            let entries = cache.window(state.base(), height as u32)?;
            Full { selected, entries }.draw(out)?;
            out.flush()?;

            let e = read()?;

            state.induce(&e);

            let action = inter.interact(&e)?;
            state.induce(&action);

            match action {
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

                Action::Quit => break,
                _ => (),
            };

            state.recalc(cache.total());
        }

        Ok(())
    }

    pub fn interact(self) -> eyre::Result<()> {
        let mut out = stdout();

        let raw = Raw::default();

        AltScreen::Enter.draw(&mut out)?;
        Clear.draw(&mut out)?;
        ShowCur(false).draw(&mut out)?;

        panic::set_hook(Box::new(|info| {
            let mut out = stdout();

            let _ = Clear.draw(&mut out);
            let _ = ShowCur(true).draw(&mut out);
            let _ = AltScreen::Exit.draw(&mut out);
            let _ = out.flush();
            let _ = terminal::disable_raw_mode();

            println!("{}", info);
        }));

        let res = self.render(&mut out);

        let _ = Clear.draw(&mut out);
        let _ = ShowCur(true).draw(&mut out);
        let _ = AltScreen::Exit.draw(&mut out)?;

        drop(raw);
        out.flush()?;

        let _ = panic::take_hook();

        res
    }
}
