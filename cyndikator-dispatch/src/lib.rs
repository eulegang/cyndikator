//!
//! A tiny DSL for deciding what to do with an action
//!

mod dispatch;
mod event;

pub use dispatch::{Action, Dispatch, ParseError};
pub use event::{Event, EventBuilder};
