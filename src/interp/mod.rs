use crate::{FeedItem, feed::FeedMeta};

mod alert;
mod exec;
mod record;

pub use alert::Alert;
pub use exec::Exec;
pub use record::Record;

#[derive(Clone, Debug)]
pub struct Program {
    pub instructions: Vec<Instruction>,
}

pub struct Interp {}

#[derive(Clone, Debug)]
pub enum Instruction {
    Alert(Alert),
    Record(Record),
    Exec(Exec),
}

impl Interp {
    pub fn run(&self, meta: &FeedMeta, item: &FeedItem, prog: &Program) -> crate::Result<()> {
        for inst in &prog.instructions {
            match inst {
                Instruction::Alert(alert) => alert.run(meta, item, self)?,
                Instruction::Record(record) => record.run(meta, item, self)?,
                Instruction::Exec(exec) => exec.run(meta, item, self)?,
            }
        }
        Ok(())
    }
}

trait InterpInst {
    fn run(&self, meta: &FeedMeta, item: &FeedItem, interp: &Interp) -> crate::Result<()>;
}

impl Program {
    pub fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for inst in &self.instructions {
            match inst {
                Instruction::Alert(alert) => writeln!(f, "  {alert}")?,
                Instruction::Record(record) => writeln!(f, "  {record}")?,
                Instruction::Exec(exec) => writeln!(f, "  {exec}")?,
            }
        }

        Ok(())
    }
}
