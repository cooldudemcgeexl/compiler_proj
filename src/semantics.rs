use thiserror::Error;

pub mod context;
pub mod traits;
pub mod value;

use crate::parser::program::ProgramStruct;

use self::value::Type;

#[derive(Debug, Error)]
pub enum SemanticsError {
    #[error("Type mismatch. Expected: {0:?}, Got: {1:?}")]
    TypeMismatch(Type, Type),
    #[error("Variable {0} redeclared within local scope.")]
    Redeclared(String),
}

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
