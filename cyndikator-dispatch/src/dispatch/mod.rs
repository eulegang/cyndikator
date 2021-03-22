use crate::Event;
use parse::Parsable;
use runtime::DispatchCase;
use std::fmt;

use token::Token;

mod parse;
mod token;

pub(crate) mod runtime;

#[cfg(test)]
mod test;

/// A set of rules to determine what [Action]s should be taken given an [Event]
#[derive(Debug)]
pub struct Dispatch {
    cases: Vec<runtime::DispatchCase>,
}

/// An action to take given a specific [Event]
#[derive(Debug, PartialEq)]
pub enum Action {
    /// Record the event for viewing later
    Record,

    /// Notify via an approprate channel (system notification system)
    Notify,

    /// Execute a shell line
    Exec(String),
}

impl Dispatch {
    /// Find the [Action]s to take given an [Event].
    pub fn dispatch(&self, event: &Event) -> Vec<Action> {
        let mut actions = Vec::with_capacity(self.cases.len());
        for case in &self.cases {
            if case.cond.satisfies(event) {
                for gen in &case.actions {
                    if gen.is_drop() {
                        return actions;
                    }

                    let action = gen.generate(event);

                    if !actions.contains(&action) {
                        actions.push(action);
                    }
                }
            }
        }

        actions
    }

    /// Parse the DSL into a [Dispatch]
    pub fn parse(input: &str) -> Result<Dispatch, ParseError> {
        let tokens = Token::tokenize_significant(input)?;

        let mut tokens = &tokens[..];
        let mut cases = Vec::new();

        while !tokens.is_empty() {
            let (t, case) = DispatchCase::parse(&tokens)?;
            cases.push(case);

            tokens = t;
        }

        Ok(Dispatch { cases })
    }
}

/// An Error found when parsing the [Dispatch] DSL
#[derive(Debug)]
#[non_exhaustive]
pub enum ParseError {
    /// An issue found during tokenization
    Tokenize,

    /// Parsing expected a token but ran out of tokens.
    EndOfTokens,

    /// Some sort of expectation failed with the arrangement of tokens
    InvalidExpectation {
        /// a description of what was expected
        expect: String,
        /// a debug representation of the token found instead of the expectation
        reality: String,
    },
}

impl std::error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::Tokenize => write!(fmt, "Issue with tokenization"),
            ParseError::EndOfTokens => write!(fmt, "Ran out of tokens"),
            ParseError::InvalidExpectation { expect, reality } => {
                write!(fmt, "expected {} but received {}", expect, reality)
            }
        }
    }
}
