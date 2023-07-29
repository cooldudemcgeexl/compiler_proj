use super::procedure::{ProcedureBody, ProcedureHeader};
use super::types::{ArrayBound, TypeMark};

pub enum Declaration {
    Procedure(ProcedureDeclaration),
    Variable(VariableDeclaration),
}

pub struct ProcedureDeclaration {
    pub procedure_header: ProcedureHeader,
    pub procedure_body: ProcedureBody,
}

pub struct VariableDeclaration {
    pub identifier: String,
    pub type_mark: TypeMark,
    pub array_bound: ArrayBound,
}
