use super::expression::Expression;
use super::types::Identifier;
pub enum Statement {
    Assignment(AssignmentStatement),
    If(IfStatement),
    Loop(LoopStatement),
    Return(ReturnStatement)
}

pub struct AssignmentStatement {
    pub destination: Destination,
    pub expression: Expression
}

pub struct IfStatement {
    pub expression: Expression,
    pub statement: Box<Statement>,
    pub else_statement: Option<Box<Statement>>
}

pub struct LoopStatement {
    pub assignment_statement: AssignmentStatement,
    pub expression: Expression,
    pub statement: Box<Statement> 
}

pub struct ReturnStatement {
    pub expression: Expression
}

pub struct Destination {
    pub identifier: Identifier,
    pub expression: Option<Expression>
}