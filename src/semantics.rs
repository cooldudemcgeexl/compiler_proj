use std::num::{ParseFloatError, ParseIntError};

use thiserror::Error;

pub mod context;
pub mod declaration;
pub mod expression;
pub mod procedure;
pub mod statement;
pub mod traits;
pub mod value;

use crate::parser::program::ProgramStruct;

use self::context::{Context, Scope, ScopeContext};
use self::procedure::AnalyzedProcedure;
use self::statement::AnalyzedBlock;
use self::traits::Analyze;
use self::value::Type;

#[derive(Debug, Error)]
pub enum SemanticsError {
    #[error("Type mismatch. Expected: {0:?}, Got: {1:?}")]
    TypeMismatch(Type, Type),
    #[error("Encountered invalid type {1:?}. Expected {0}.")]
    InvalidType(String, Type),

    #[error("Encountered {0} params. Expected {1}")]
    ParamCountMismatch(usize, usize),

    #[error("Variable {0} redeclared within local scope.")]
    Redeclared(String),
    #[error("Undeclared reference {0}")]
    UndefinedRef(String),
    #[error("Reached end of scope")]
    OutOfScope,
    #[error("Attempted to index a non-array object {0}.")]
    IndexOnNonArray(String),
    #[error("Attempted to index array {0} using non-integer index of type {1:?}")]
    NonIntIndex(String, Type),
    #[error("Encountered return when none was expected.")]
    UnexpectedReturn,

    #[error(transparent)]
    InvalidIntLiteral(#[from] ParseIntError),
    #[error(transparent)]
    InvalidFloatLiteral(#[from] ParseFloatError),
}

#[derive(Debug)]
pub struct AnalyzedProgram {
    pub name: String,
    pub declarations: ScopeContext,
    pub procedures: Vec<AnalyzedProcedure>,
    pub block: AnalyzedBlock,
}

impl AnalyzedProgram {
    pub fn analyze(program: ProgramStruct) -> Result<Self, SemanticsError> {
        let mut context = Context::new();

        let name = program.program_header.header_identifier;
        let mut procedures = Vec::new();

        for declaration in program.program_body.declarations {
            if let Some(procedure) = declaration.analyze(&mut context, &Scope::Global)? {
                procedures.push(procedure);
            }
        }

        let block = program
            .program_body
            .statements
            .analyze(&mut context, &Scope::Local)?;

        Ok(AnalyzedProgram {
            name,
            declarations: context.into_global(),
            procedures,
            block,
        })
    }
}
