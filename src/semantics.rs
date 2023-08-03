use thiserror::Error;

pub mod context;
pub mod expression;
pub mod statement;
pub mod traits;
pub mod value;

use crate::parser::program::ProgramStruct;

use self::context::{Context, ScopeContext};
use self::value::Type;

#[derive(Debug, Error)]
pub enum SemanticsError {
    #[error("Type mismatch. Expected: {0:?}, Got: {1:?}")]
    TypeMismatch(Type, Type),
    #[error("Variable {0} redeclared within local scope.")]
    Redeclared(String),
    #[error("Undeclared reference {0}")]
    UndefinedRef(String),
    #[error("Reached end of scope")]
    OutOfScope,
    #[error("Attempted to index a non-array object {0}.")]
    IndexOnNonArray(String),
}

#[derive(Debug)]
pub struct AnalyzedProgram {
    pub name: String,
    pub declarations: ScopeContext,
    pub procedures: Vec<()>,
    pub block: (),
}

impl AnalyzedProgram {
    pub fn analyze(val: ProgramStruct) -> Result<Self, SemanticsError> {
        let mut context = Context::new();
        todo!()
    }
}
