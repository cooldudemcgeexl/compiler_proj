use super::declaratons::{Declaration, VariableDeclaration};
use super::expression::Expression;
use super::statement::Statement;
use super::types::{Identifier, TypeMark};

pub struct ProcedureHeader {
    pub identifier: String,
    pub type_mark: TypeMark,
    pub param_list: Option<ParamList>,
}

pub struct ProcedureBody {
    pub declarations: Vec<Declaration>,
    pub statements: Option<Vec<Statement>>,
}

pub struct ParamList {
    pub param_list: Vec<Parameter>,
}

pub struct Parameter {
    pub variable_declaration: VariableDeclaration,
}
pub struct ProcedureCall {
    pub identifier: Identifier,
    pub arg_list: Option<ArgumentList>,
}

pub struct ArgumentList {
    pub expr_list: Vec<Expression>,
}
