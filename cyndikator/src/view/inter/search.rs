use super::{Action, Mode};
use crossterm::{
    event::{KeyCode, KeyEvent},
    Result,
};

pub struct Search {
    query: String,
}

impl Default for Search {
    fn default() -> Search {
        let query = String::with_capacity(256);
        Search { query }
    }
}

impl Mode for Search {
    fn handle(&mut self, event: &KeyEvent) -> Result<Action> {
        let action = match event.code {
            KeyCode::Enter => Action::SetSearch(self.query.clone()),
            KeyCode::Backspace => {
                self.query.pop();
                Action::SearchPreview(self.query.clone())
            }
            KeyCode::Char(ch) => {
                self.query.push(ch);
                Action::SearchPreview(self.query.clone())
            }

            _ => Action::Noop,
        };

        Ok(action)
    }

    fn status(&self) -> Option<String> {
        Some(format!("/{}", self.query))
    }
}
