use crossterm::{QueueableCommand, Result};

mod alt;
mod clear;
mod cur;
mod full;

pub use alt::AltScreen;
pub use clear::Clear;
pub use cur::ShowCur;
pub use full::Full;

pub trait Draw {
    fn draw(&self, out: &mut impl QueueableCommand) -> Result<()>;
}
