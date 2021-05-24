use crate::db::Database;
use crossterm::{
    event::{read, Event},
    terminal,
};
use std::io::{stdout, Write};
use std::panic;

use draw::*;
use inter::Mode;
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

pub struct Indexes(Vec<u32>);

pub enum Action {
    RelUp(u16, ScrollUnit),
    RelDown(u16, ScrollUnit),
    Goto(Position),
    Delete,
    Undo,
    Open,
    Noop,
    Quit,

    StartSearch,
    SearchPreview(String),
    SetSearch(String),
    Next,
    Prev,
}

impl View {
    pub fn new(db: Database) -> View {
        View { db }
    }

    fn render(self, out: &mut impl Write) -> eyre::Result<()> {
        let mut state = State::new(terminal::size()?.1);
        let mut cache = Cache::new(self.db);
        let mut inter = inter::Inter::default();

        loop {
            let selected = state.offset();
            let entries = cache.window(state.base(), state.height() as u32)?;
            Full {
                selected,
                entries,
                status: inter.status(),
            }
            .draw(out)?;
            out.flush()?;

            let e = read()?;
            state.induce(&e);

            let action = match e {
                Event::Key(e) => inter.handle(&e)?,
                _ => {
                    state.recalc(cache.total());
                    continue;
                }
            };
            state.induce(&action);

            match action {
                Action::Open => {
                    if let Some(url) = cache.hot_load(state.abs()).and_then(|e| e.url.as_ref()) {
                        let _ = open::that_in_background(url);
                    }
                }

                Action::Delete => {
                    cache.delete(state.abs());
                }

                Action::Undo => {
                    cache.undo();
                }

                Action::Quit => break,

                Action::SetSearch(ref search) => {
                    let idx = &cache.search_indexes(search)?;
                    state.goto_next(idx);
                }

                Action::Next => {
                    if let Some(search) = state.search() {
                        let idx = &cache.search_indexes(search)?;
                        state.goto_next(idx);
                    }
                }

                Action::Prev => {
                    if let Some(search) = state.search() {
                        let idx = &cache.search_indexes(search)?;
                        state.goto_prev(idx);
                    }
                }

                _ => (),
            };

            inter.induce(&action);

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

impl Indexes {
    fn next(&self, idx: u32) -> Option<u32> {
        for pos in &self.0 {
            if *pos > idx {
                return Some(*pos);
            }
        }

        None
    }

    fn prev(&self, idx: u32) -> Option<u32> {
        let mut res = None;
        for pos in &self.0 {
            if *pos > idx {
                return res;
            }

            if *pos < idx {
                res = Some(*pos);
            }
        }

        res
    }
}
