use crate::{
    FeedItem,
    feed::FeedMeta,
    interp::{Instruction, InterpInst},
};

#[derive(Debug, Clone)]
pub struct Record {}

impl InterpInst for Record {
    fn run(&self, _: &FeedMeta, _: &FeedItem, _: &super::Interp) -> crate::Result<()> {
        Ok(())
    }
}

impl std::fmt::Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "record")
    }
}

impl From<Record> for Instruction {
    fn from(value: Record) -> Self {
        Instruction::Record(value)
    }
}
