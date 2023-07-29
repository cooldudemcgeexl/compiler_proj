use super::types::{TypeMark, ArrayBound};
use super::procedure::{ProcedureHeader,ProcedureBody};

pub enum Declaration {
    Procedure(ProcedureDeclaration),
    Variable(VariableDeclaration)
}

pub struct ProcedureDeclaration {
    pub procedure_header: ProcedureHeader,
    pub procedure_body: ProcedureBody
}

pub struct VariableDeclaration { 
    pub identifier: String,
    pub type_mark: TypeMark,
    pub array_bound: ArrayBound
}


