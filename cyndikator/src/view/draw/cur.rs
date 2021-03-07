use super::*;
use crossterm::cursor::{Hide, Show};

pub struct ShowCur(pub bool);

impl Draw for ShowCur {
    fn draw(&self, out: &mut impl QueueableCommand) -> Result<()> {
        if self.0 {
            out.queue(Show)?;
        } else {
            out.queue(Hide)?;
        }

        Ok(())
    }
}
