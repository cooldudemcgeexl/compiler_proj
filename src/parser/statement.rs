use super::expression::Expression;
use super::types::Identifier;
#[derive(Debug)]
pub enum Statement {
    Assignment(AssignmentStatement),
    If(IfStatement),
    Loop(LoopStatement),
    Return(ReturnStatement),
}

#[derive(Debug)]
pub struct AssignmentStatement {
    pub destination: Destination,
    pub expression: Expression,
}

#[derive(Debug)]
pub struct IfStatement {
    pub expression: Expression,
    pub statement: Box<Statement>,
    pub else_statement: Option<Box<Statement>>,
}

#[derive(Debug)]
pub struct LoopStatement {
    pub assignment_statement: AssignmentStatement,
    pub expression: Expression,
    pub statement: Box<Statement>,
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub expression: Expression,
}

#[derive(Debug)]
pub struct Destination {
    pub identifier: Identifier,
    pub expression: Option<Expression>,
}
