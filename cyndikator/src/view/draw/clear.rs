use super::*;

pub struct Clear;

impl Draw for Clear {
    fn draw(&self, out: &mut impl QueueableCommand) -> Result<()> {
        use crossterm::terminal::{Clear, ClearType};
        out.queue(Clear(ClearType::All))?;

        Ok(())
    }
}
