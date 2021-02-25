use super::Event;
use crate::db::Database;
use std::str::FromStr;

use nom::{branch::alt, bytes::complete::tag, combinator::all_consuming, IResult};

pub enum Action {
    Notify,
    Shell(String),
    Record,
}

impl Action {
    pub fn act(&self, event: &Event, db: &mut Database) {
        todo!()
    }
}

impl FromStr for Action {
    type Err = ();

    fn from_str(input: &str) -> Result<Action, ()> {
        all_consuming(parse)(input).or(Err(())).map(|s| s.1)
    }
}

fn parse(input: &str) -> IResult<&str, Action> {
    alt((parse_shell, parse_lit))(input)
}

fn parse_shell(input: &str) -> IResult<&str, Action> {
    let (input, text) = tag("!")(input)?;

    Ok(("", Action::Shell(input.to_string())))
}

fn parse_lit(input: &str) -> IResult<&str, Action> {
    let (input, text) = alt((
        tag("notify"),
        tag("record"),
        tag("note"),
        tag("rec"),
        tag("n"),
        tag("r"),
    ))(input)?;

    Ok((
        input,
        match text {
            "notify" | "note" | "n" => Action::Notify,
            "record" | "rec" | "r" => Action::Record,
            _ => unreachable!(),
        },
    ))
}
