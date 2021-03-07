use super::*;
use crate::db::Entry;
use crossterm::style::{Color, Print, SetForegroundColor};
use crossterm::{cursor, terminal};
use std::borrow::Cow;

pub struct Full<'a> {
    pub selected: u16,
    pub entries: &'a [Entry],
}

impl<'a> Draw for Full<'a> {
    fn draw(&self, out: &mut impl QueueableCommand) -> Result<()> {
        let (width, height) = terminal::size()?;

        let end = (height as usize).min(self.entries.len());
        let rem = (height as usize)
            .checked_sub(self.entries.len())
            .unwrap_or(0);

        let ents = &self.entries[..end];

        out.queue(cursor::MoveTo(0, 0))?;
        out.queue(terminal::Clear(terminal::ClearType::All))?;

        let mut i = 0;
        for entry in ents {
            Line {
                width,
                selected: i == self.selected,
                entry,
            }
            .draw(out)?;
            out.queue(cursor::MoveToNextLine(1))?;
            i += 1;
        }

        out.queue(SetForegroundColor(Color::Blue))?;
        for _ in 0..rem {
            out.queue(Print("~"))?;
            out.queue(cursor::MoveToNextLine(1))?;
        }
        out.queue(SetForegroundColor(Color::Reset))?;

        Ok(())
    }
}

struct Line<'a> {
    width: u16,
    selected: bool,
    entry: &'a Entry,
}

impl<'a> Draw for Line<'a> {
    fn draw(&self, out: &mut impl QueueableCommand) -> Result<()> {
        let cat_full = self.entry.categories.join(", ");

        let feed = trunc(
            &self.entry.feed.as_deref().unwrap_or("<untitled feed>"),
            self.width / 4 - 2,
        );
        let title = trunc(
            &self.entry.title.as_deref().unwrap_or("<untitled item>"),
            self.width / 2,
        );
        let cat = trunc(&cat_full, self.width / 4);

        if self.selected {
            out.queue(SetForegroundColor(Color::Yellow))?;
            out.queue(Print("* "))?;
        } else {
            out.queue(Print("  "))?;
        }

        out.queue(SetForegroundColor(Color::Blue))?;
        out.queue(Print(&feed))?;

        out.queue(cursor::MoveToColumn(self.width / 4))?;
        out.queue(SetForegroundColor(Color::Cyan))?;
        out.queue(Print(&title))?;

        out.queue(cursor::MoveToColumn(self.width - cat.len() as u16))?;
        out.queue(SetForegroundColor(Color::Green))?;
        out.queue(Print(&cat))?;

        out.queue(SetForegroundColor(Color::Reset))?;

        Ok(())
    }
}

/// Simple and wrong but want to move one TODO: fixup
/// Assumes ascii not utf8
fn trunc<'a>(input: &'a str, width: u16) -> Cow<'a, str> {
    if input.len() <= width as usize {
        input.into()
    } else {
        let mut buf = input[0..width as usize].to_string();
        buf.pop();
        buf.pop();
        buf.pop();

        for _ in 0..width - buf.len() as u16 {
            buf.push('.');
        }

        buf.into()
    }
}

#[test]
fn trunc_test() {
    assert_eq!(trunc("hello", 5), "hello");
    assert_eq!(trunc("hello", 4), "h...");
    assert_eq!(trunc("hello", 2), "..");
}
