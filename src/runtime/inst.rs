#[derive(Clone, Debug)]
pub struct Program {
    pub instructions: Vec<Instruction>,
}

#[derive(Clone, Debug)]
pub enum Instruction {
    Alert,
    Record,
    Exec(String),
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
                Instruction::Alert => writeln!(f, "  alert")?,
                Instruction::Record => writeln!(f, "  record")?,
                Instruction::Exec(cmd) => writeln!(f, "  exec `{}`", cmd)?,
            }
        }

        Ok(())
    }
}
