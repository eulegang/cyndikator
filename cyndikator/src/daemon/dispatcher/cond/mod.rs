use super::Event;
use nom::{combinator::all_consuming, IResult};
use std::str::FromStr;

pub enum Condition {
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
    Not(Box<Condition>),
    In(Expr, Expr),
}

impl Condition {
    pub fn satisfied(&self, event: &Event) -> bool {
        todo!();
    }
}

impl FromStr for Condition {
    type Err = ();

    fn from_str(input: &str) -> Result<Condition, ()> {
        all_consuming(parse)(input).or(Err(())).map(|s| s.1)
    }
}

fn parse(input: &str) -> IResult<&str, Condition> {
    todo!()
}
