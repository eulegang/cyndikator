//!
//! This is the best approximation to the atom standard for my use case
//! eventually I will be reworking this crate
//!
//! This crate
//! * does not support xhtml in text elements
//! * may be less strict than the standard
//! * does not support extensions
//! * some semantic expectations may be violated
//!

#[cfg(test)]
mod test;

mod errs;
mod impls;
mod types;

pub use errs::*;
pub use types::*;
