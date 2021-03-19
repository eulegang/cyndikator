use super::Action;
use crate::Event;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct DispatchCase {
    pub cond: Condition,
    pub actions: Vec<ActionGen>,
}

#[derive(Debug, PartialEq)]
pub enum Condition {
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
    Not(Box<Condition>),
    Op(Op),
}

#[derive(Debug, PartialEq)]
pub enum ActionGen {
    Drop,
    Notify,
    Record,
    Exec(StringInterpol),
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Is(Expr, Expr),
    In(Expr, Expr),
    Matches(Expr, Re),
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Str(StringInterpol),
    Var(Var),
    Null,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Nil,
    Str(String),
    Strs(Vec<String>),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Var {
    URL,
    Title,
    Categories,
    Description,
    FeedURL,
    FeedTitle,
    FeedCategories,
}

#[derive(Debug, PartialEq)]
pub enum StringInterpol {
    Inert(String),
    Live { lits: Vec<String>, vars: Vec<Var> },
}

#[derive(Debug)]
pub struct Re {
    regex: Regex,
}

impl PartialEq for Re {
    fn eq(&self, other: &Re) -> bool {
        self.regex.as_str() == other.regex.as_str()
    }
}

impl From<Regex> for Re {
    fn from(regex: Regex) -> Re {
        Re { regex }
    }
}

impl std::ops::Deref for Re {
    type Target = Regex;
    fn deref(&self) -> &Regex {
        &self.regex
    }
}

impl Var {
    pub fn realize(&self, event: &Event) -> Value {
        match self {
            Var::URL => event.url.clone().into(),
            Var::Title => event.title.clone().into(),
            Var::Categories => event.categories.clone().into(),
            Var::Description => event.description.clone().into(),

            Var::FeedURL => event.feed_url.clone().into(),
            Var::FeedTitle => event.feed_title.clone().into(),
            Var::FeedCategories => event.feed_categories.clone().into(),
        }
    }
}

impl Expr {
    pub fn realize(&self, event: &Event) -> Value {
        match self {
            Expr::Str(s) => Value::Str(s.interpolate(event)),
            Expr::Var(v) => v.realize(event),
            Expr::Null => Value::Nil,
        }
    }
}

impl From<Option<String>> for Value {
    fn from(os: Option<String>) -> Value {
        match os {
            Some(s) => Value::Str(s.clone()),
            None => Value::Nil,
        }
    }
}

impl From<Vec<String>> for Value {
    fn from(ss: Vec<String>) -> Value {
        Value::Strs(ss)
    }
}

impl From<String> for Value {
    fn from(s: String) -> Value {
        Value::Str(s)
    }
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::Nil => String::new(),
            Value::Str(s) => s.clone(),
            Value::Strs(s) => s.join(" "),
        }
    }
}

impl StringInterpol {
    pub fn interpolate(&self, event: &Event) -> String {
        match self {
            StringInterpol::Inert(s) => s.clone(),
            StringInterpol::Live { lits, vars } => {
                let mut buf = String::new();
                let mut i = 0;

                while i < vars.len() {
                    buf.push_str(&lits[i]);
                    buf.push_str(&vars[i].realize(event).to_string());
                    i += 1;
                }

                buf.push_str(&lits[i]);

                buf
            }
        }
    }
}

impl Condition {
    pub fn satisfies(&self, event: &Event) -> bool {
        match self {
            Condition::And(a, b) => a.satisfies(event) && b.satisfies(event),
            Condition::Or(a, b) => a.satisfies(event) || b.satisfies(event),
            Condition::Not(c) => !c.satisfies(event),
            Condition::Op(op) => op.satisfies(event),
        }
    }
}

impl Op {
    fn satisfies(&self, event: &Event) -> bool {
        match self {
            Op::Is(l, r) => match (l.realize(event), r.realize(event)) {
                (Value::Nil, Value::Nil) => true,
                (Value::Str(l), Value::Str(r)) => l == r,
                (Value::Strs(l), Value::Strs(r)) => l == r,
                (_, _) => false,
            },
            Op::In(l, r) => match (l.realize(event), r.realize(event)) {
                (Value::Nil, Value::Nil) => true,
                (Value::Nil, _) => false,
                (_, Value::Nil) => false,

                (Value::Str(s), Value::Strs(ss)) => ss.iter().any(|sub| &s == sub),
                (Value::Str(l), Value::Str(r)) => r.find(&l).is_some(),

                (Value::Strs(l), Value::Str(r)) => l.iter().any(|ls| ls == &r),
                (Value::Strs(l), Value::Strs(r)) => l.iter().all(|ls| r.iter().any(|lr| ls == lr)),
            },
            Op::Matches(e, re) => match e.realize(event) {
                Value::Nil => false,
                Value::Str(s) => re.is_match(&s),
                Value::Strs(ss) => ss.iter().any(|s| re.is_match(s)),
            },
        }
    }
}

impl ActionGen {
    pub fn is_drop(&self) -> bool {
        matches!(self, ActionGen::Drop)
    }

    pub fn generate(&self, event: &Event) -> Action {
        match self {
            ActionGen::Drop => unreachable!("Should never call generate on drop"),
            ActionGen::Notify => Action::Notify,
            ActionGen::Record => Action::Record,
            ActionGen::Exec(si) => Action::Exec(si.interpolate(event)),
        }
    }
}
