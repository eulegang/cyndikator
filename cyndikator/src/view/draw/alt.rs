use super::*;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};

pub enum AltScreen {
    Enter,
    Exit,
}

impl Draw for AltScreen {
    fn draw(&self, out: &mut impl QueueableCommand) -> Result<()> {
        match self {
            AltScreen::Enter => out.queue(EnterAlternateScreen),
            AltScreen::Exit => out.queue(LeaveAlternateScreen),
        }?;

        Ok(())
    }
}
