use crossterm::{QueueableCommand, Result};

mod clear;
mod cur;
mod full;

pub use clear::Clear;
pub use cur::ShowCur;
pub use full::Full;

pub trait Draw {
    fn draw(&self, out: &mut impl QueueableCommand) -> Result<()>;
}
