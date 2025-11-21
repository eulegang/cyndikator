use crate::{FeedItem, feed::FeedMeta, interp::inst::Program};

pub(crate) mod inst;

pub struct Interp {}

impl Interp {
    pub fn run(&self, meta: &FeedMeta, item: &FeedItem, prog: &Program) -> crate::Result<()> {
        for inst in &prog.instructions {
            match inst {
                inst::Instruction::Alert => self.alert(meta, item)?,
                inst::Instruction::Record => self.record(meta, item)?,
                inst::Instruction::Exec(sh) => self.exec(meta, item, sh)?,
            }
        }
        Ok(())
    }

    fn alert(&self, meta: &FeedMeta, item: &FeedItem) -> crate::Result<()> {
        todo!()
    }

    fn record(&self, meta: &FeedMeta, item: &FeedItem) -> crate::Result<()> {
        todo!()
    }

    fn exec(&self, meta: &FeedMeta, item: &FeedItem, sh: &str) -> crate::Result<()> {
        todo!()
    }
}
