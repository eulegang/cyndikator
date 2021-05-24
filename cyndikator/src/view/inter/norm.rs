use super::{Action, Mode, Position, ScrollUnit};
use crossterm::{
    event::{KeyCode, KeyEvent},
    Result,
};

pub struct Norm {
    mag: String,
}

impl Default for Norm {
    fn default() -> Norm {
        let mag = String::with_capacity(16);
        Norm { mag }
    }
}

impl Mode for Norm {
    fn handle(&mut self, event: &KeyEvent) -> Result<Action> {
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

            KeyCode::Char('n') => Action::Next,
            KeyCode::Char('N') => Action::Prev,

            KeyCode::Char('/') => Action::StartSearch,
            KeyCode::Enter => Action::Open,

            KeyCode::Char(ch) if ('0'..='9').contains(&ch) => {
                self.mag.push(ch);
                Action::Noop
            }

            _ => Action::Noop,
        };

        Ok(action)
    }

    fn status(&self) -> Option<String> {
        None
    }
}

fn clearable(code: &KeyCode) -> bool {
    if let KeyCode::Char(ch) = code {
        ('0'..='9').contains(ch)
    } else {
        false
    }
}
