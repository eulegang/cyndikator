use crossterm::terminal;

pub struct Raw;

impl Default for Raw {
    fn default() -> Raw {
        let _ = terminal::enable_raw_mode();

        Raw
    }
}

impl Drop for Raw {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
    }
}
