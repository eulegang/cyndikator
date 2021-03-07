use crossterm::{
    event::{Event, KeyCode, KeyEvent},
    Result,
};

use super::Action;

pub struct Inter {
    pub height: u16,
    pub offset: u16,
    pub base: u32,
    pub mag: String,
}

impl Inter {
    pub fn new(height: u16, base: u32) -> Inter {
        let offset = 0;
        let mag = String::new();

        Inter {
            offset,
            height,
            base,
            mag,
        }
    }

    pub fn interact(&mut self, event: &Event) -> Result<Action> {
        match event {
            Event::Key(ke) => return self.inter_key(ke),

            Event::Resize(_, height) => self.height = *height,

            _ => (),
        };

        Ok(Action::Noop)
    }

    fn inter_key(&mut self, event: &KeyEvent) -> Result<Action> {
        let mag = self.mag.parse::<u16>().ok();

        match event.code {
            KeyCode::Char('q') => return Ok(Action::Quit),
            KeyCode::Char('j') | KeyCode::Down => {
                self.move_down(mag.unwrap_or(1));
                self.mag.clear();
            }

            KeyCode::Char('k') | KeyCode::Up => {
                self.move_up(mag.unwrap_or(1));
                self.mag.clear();
            }

            KeyCode::Char('g') => {
                self.goto(mag.unwrap_or(1) as u32);
                self.mag.clear();
            }

            KeyCode::Char('G') => {
                self.offset = mag
                    .map(|i| i.checked_sub(1).unwrap_or(1))
                    .unwrap_or(u16::MAX);
                self.mag.clear();
            }

            KeyCode::Char('d') => {
                self.move_down(mag.unwrap_or(1) * self.height / 2);
                self.mag.clear();
            }

            KeyCode::Char('u') => {
                self.move_up(mag.unwrap_or(1) * self.height / 2);
                self.mag.clear();
            }

            KeyCode::Char('f') => {
                self.move_down(mag.unwrap_or(1) * self.height);
                self.mag.clear();
            }

            KeyCode::Char('b') => {
                self.move_up(mag.unwrap_or(1) * self.height);
                self.mag.clear();
            }

            KeyCode::Char('D') => {
                self.mag.clear();
                return Ok(Action::Delete);
            }

            KeyCode::Char('U') => {
                self.mag.clear();
                return Ok(Action::Undo);
            }

            KeyCode::Char(ch) if ('0'..='9').contains(&ch) => {
                self.mag.push(ch);
            }

            KeyCode::Enter => {
                self.mag.clear();
                return Ok(Action::Open);
            }

            _ => (),
        };

        Ok(Action::Noop)
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
        self.offset = line.checked_sub(1).unwrap_or(0) as u16;
    }
}
