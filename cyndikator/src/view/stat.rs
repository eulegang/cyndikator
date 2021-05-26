use super::{Action, Indexes, Inducable, Position, ScrollUnit};
use crossterm::event::Event;

pub struct State {
    height: u16,
    offset: u16,
    base: u32,

    search: Option<String>,
}

impl State {
    pub fn new(height: u16) -> State {
        let offset = 0;
        let base = 0;
        let search = None;

        State {
            height,
            offset,
            base,
            search,
        }
    }

    pub fn offset(&self) -> u16 {
        self.offset
    }

    pub fn base(&self) -> u32 {
        self.base
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn abs(&self) -> u32 {
        self.base + self.offset as u32
    }

    pub fn search(&self) -> Option<&str> {
        self.search.as_deref()
    }

    pub fn recalc(&mut self, total: u32) {
        if self.offset as u32 + self.base >= total {
            self.base = total.saturating_sub(self.height as u32);

            self.offset = total.saturating_sub(self.base).saturating_sub(1) as u16;
        }

        if self.offset >= self.height {
            let diff = self.offset - self.height + 1;
            self.offset = self.offset.saturating_sub(diff);
            self.base += diff as u32;
        }
        self.offset = self.offset.min(total as u16 - 1);
    }

    pub fn goto_next(&mut self, idx: &Indexes) {
        if let Some(next) = idx.next(self.abs() + 1) {
            self.goto(next);
        }
    }

    pub fn goto_prev(&mut self, idx: &Indexes) {
        if let Some(next) = idx.prev(self.abs() + 1) {
            self.goto(next);
        }
    }

    fn move_down(&mut self, amount: u16) {
        self.offset += amount;

        if self.offset >= self.height {
            self.base += (self.offset - self.height + 1) as u32;
            self.offset = self.height - 1;
        }
    }

    fn move_up(&mut self, amount: u16) {
        let diff = if amount > self.offset {
            amount - self.offset
        } else {
            0
        };

        self.offset = self.offset.saturating_sub(amount);
        self.base = self.base.saturating_sub(diff as u32);
    }

    fn goto(&mut self, line: u32) {
        let adjusted = line.saturating_sub(1);

        if adjusted < self.base {
            self.base = adjusted;
            self.offset = 0;
        } else {
            self.offset = (adjusted - self.base) as u16;
        }
    }
}

impl Inducable<Action> for State {
    fn induce(&mut self, elem: &Action) {
        let half = self.height >> 1;
        match elem {
            Action::RelUp(amount, ScrollUnit::Line) => self.move_up(*amount),
            Action::RelDown(amount, ScrollUnit::Line) => self.move_down(*amount),
            Action::RelUp(amount, ScrollUnit::Half) => self.move_up(*amount * half),
            Action::RelDown(amount, ScrollUnit::Half) => self.move_down(*amount * half),
            Action::RelUp(amount, ScrollUnit::Page) => self.move_up(*amount * self.height),
            Action::RelDown(amount, ScrollUnit::Page) => self.move_down(*amount * self.height),
            Action::Goto(Position::Abs(line)) => self.goto(*line),
            Action::Goto(Position::Last) => self.goto(u16::MAX.into()),

            Action::SetSearch(s) => self.search = Some(s.clone()),

            _ => (),
        }
    }
}

impl Inducable<Event> for State {
    fn induce(&mut self, elem: &Event) {
        if let Event::Resize(_, height) = elem {
            self.height = *height;
        }
    }
}
