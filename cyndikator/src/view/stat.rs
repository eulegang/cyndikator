use super::{Action, Inducable, Position, ScrollUnit};
use crossterm::event::Event;

pub struct State {
    height: u16,
    offset: u16,
    base: u32,
}

impl State {
    pub fn new(height: u16) -> State {
        let offset = 0;
        let base = 0;

        State {
            height,
            offset,
            base,
        }
    }

    pub fn offset(&self) -> u16 {
        self.offset
    }

    pub fn base(&self) -> u32 {
        self.base
    }

    pub fn abs(&self) -> u32 {
        self.base + self.offset as u32
    }

    pub fn recalc(&mut self, total: u32) {
        if self.offset as u32 + self.base >= total {
            self.base = total.checked_sub(self.height as u32).unwrap_or(0);

            self.offset = total
                .checked_sub(self.base)
                .unwrap_or(0)
                .checked_sub(1)
                .unwrap_or(0) as u16;
        }
        self.offset = self.offset.min(total as u16 - 1);
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

        self.offset = self.offset.checked_sub(amount).unwrap_or(0);
        self.base = self.base.checked_sub(diff as u32).unwrap_or(0);
    }

    fn goto(&mut self, line: u32) {
        let adjusted = line.checked_sub(1).unwrap_or(0);

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
        match elem {
            Action::RelUp(amount, ScrollUnit::Line) => self.move_up(*amount),
            Action::RelDown(amount, ScrollUnit::Line) => self.move_down(*amount),
            Action::RelUp(amount, ScrollUnit::Half) => self.move_up(*amount * self.height >> 1),
            Action::RelDown(amount, ScrollUnit::Half) => self.move_down(*amount * self.height >> 1),
            Action::RelUp(amount, ScrollUnit::Page) => self.move_up(*amount * self.height),
            Action::RelDown(amount, ScrollUnit::Page) => self.move_down(*amount * self.height),
            Action::Goto(Position::Abs(line)) => self.goto(*line),
            Action::Goto(Position::Last) => self.goto(u16::MAX.into()),
            _ => (),
        }
    }
}

impl Inducable<Event> for State {
    fn induce(&mut self, elem: &Event) {
        match elem {
            Event::Resize(_, height) => self.height = *height,
            _ => (),
        }
    }
}
