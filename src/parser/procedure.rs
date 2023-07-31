use super::declaratons::{Declaration, VariableDeclaration};
use super::expression::Expression;
use super::statement::Statement;
use super::types::{Identifier, TypeMark};
#[derive(Debug)]
pub struct ProcedureHeader {
    pub identifier: String,
    pub type_mark: TypeMark,
    pub param_list: Option<ParamList>,
}
#[derive(Debug)]
pub struct ProcedureBody {
    pub declarations: Vec<Declaration>,
    pub statements: Option<Vec<Statement>>,
}
#[derive(Debug)]
pub struct ParamList {
    pub param_list: Vec<Parameter>,
}
#[derive(Debug)]
pub struct Parameter {
    pub variable_declaration: VariableDeclaration,
}
#[derive(Debug)]
pub struct ProcedureCall {
    pub identifier: Identifier,
    pub arg_list: Option<ArgumentList>,
}
#[derive(Debug)]
pub struct ArgumentList {
    pub expr_list: Vec<Expression>,
}
