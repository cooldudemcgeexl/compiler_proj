use super::types::TypeMark;


pub enum Declaration {
    ProcedureDeclaration {},
    VariableDeclaration {}
}

pub struct VariableDeclaration { 
    pub identifier: String,
    pub type_mark: TypeMark,
    pub array_bound: ()
}


