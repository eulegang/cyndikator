use crossterm::{
    event::{Event, KeyCode, KeyEvent},
    Result,
};

use super::{Action, Position, ScrollUnit};

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
        let def_mag = mag.unwrap_or(1);

        if clearable(&event.code) {
            self.mag.clear();
        }

        let action = match event.code {
            KeyCode::Char('q') => Action::Quit,
            KeyCode::Char('j') | KeyCode::Down => Action::RelDown(def_mag, ScrollUnit::Line),
            KeyCode::Char('k') | KeyCode::Up => Action::RelUp(def_mag, ScrollUnit::Line),
            KeyCode::Char('g') => Action::Goto(Position::Abs(def_mag.into())),

            KeyCode::Char('d') => Action::RelDown(def_mag, ScrollUnit::Half),
            KeyCode::Char('u') => Action::RelUp(def_mag, ScrollUnit::Half),
            KeyCode::Char('f') => Action::RelDown(def_mag, ScrollUnit::Page),
            KeyCode::Char('b') => Action::RelUp(def_mag, ScrollUnit::Page),

            KeyCode::Char('D') => Action::Delete,
            KeyCode::Char('U') => Action::Undo,
            KeyCode::Char('G') => {
                let pos = mag
                    .map(|i| Position::Abs(i.checked_sub(1).unwrap_or(1).into()))
                    .unwrap_or(Position::Last);

                Action::Goto(pos)
            }
            KeyCode::Enter => Action::Open,

            KeyCode::Char(ch) if ('0'..='9').contains(&ch) => {
                self.mag.push(ch);
                Action::Noop
            }

            _ => Action::Noop,
        };

        Ok(action)
    }
}

fn clearable(code: &KeyCode) -> bool {
    if let KeyCode::Char(ch) = code {
        ('0'..='9').contains(ch)
    } else {
        false
    }
}
