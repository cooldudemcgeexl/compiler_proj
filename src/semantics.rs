use thiserror::Error;

use crate::parser::program::ProgramStruct;

#[derive(Debug, Error)]
pub enum SemanticsError {}

#[derive(Debug)]
pub struct AnalyzedProgram {
    pub name: String,
    pub declarations: (),
    pub procedures: (),
    pub block: (),
}

impl AnalyzedProgram {
    pub fn analyze(val: ProgramStruct) -> Result<Self, SemanticsError> {
        todo!()
    }
}
