use std::process::Command;

use crate::{
    FeedItem,
    feed::FeedMeta,
    interp::{Instruction, InterpInst},
};

#[derive(Debug, Clone)]
pub struct Exec {
    pub sh: String,
}

impl InterpInst for Exec {
    fn run(&self, _: &FeedMeta, _: &FeedItem, _: &super::Interp) -> crate::Result<()> {
        let _ = Command::new("sh").arg("-c").arg(&self.sh).spawn();

        Ok(())
    }
}

impl std::fmt::Display for Exec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "exec `{}`", self.sh)
    }
}

impl From<Exec> for Instruction {
    fn from(value: Exec) -> Self {
        Instruction::Exec(value)
    }
}
