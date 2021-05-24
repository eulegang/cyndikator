use super::{Action, Inducable, Position, ScrollUnit};
use crossterm::{event::KeyEvent, Result};

mod norm;
mod search;

pub use norm::Norm;
pub use search::Search;

pub enum Inter {
    Norm(Norm),
    Search(Search),
}

pub trait Mode {
    fn handle(&mut self, event: &KeyEvent) -> Result<Action>;
    fn status(&self) -> Option<String>;
}

impl Default for Inter {
    fn default() -> Inter {
        Inter::Norm(Norm::default())
    }
}

impl Inducable<Action> for Inter {
    fn induce(&mut self, action: &Action) {
        match action {
            Action::StartSearch => {
                *self = Inter::Search(Search::default());
            }

            Action::SetSearch(_) => {
                *self = Inter::Norm(Norm::default());
            }

            _ => (),
        }
    }
}

impl Mode for Inter {
    fn handle(&mut self, event: &KeyEvent) -> Result<Action> {
        match self {
            Inter::Norm(norm) => norm.handle(event),
            Inter::Search(search) => search.handle(event),
        }
    }

    fn status(&self) -> Option<String> {
        match self {
            Inter::Norm(norm) => norm.status(),
            Inter::Search(search) => search.status(),
        }
    }
}
