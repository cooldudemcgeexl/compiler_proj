use super::procedure::ProcedureCall;
use super::types::*;

#[derive(Debug)]
pub enum Expression {
    AndExp(AndExpression),
    OrExp(OrExpression),
    BasicExp(BasicExp),
}

#[derive(Debug)]
pub struct AndExpression {
    pub expression: Box<Expression>,
    pub arith_op: Box<ArtihOp>,
}

#[derive(Debug)]
pub struct OrExpression {
    pub expression: Box<Expression>,
    pub arith_op: Box<ArtihOp>,
}

#[derive(Debug)]
pub struct BasicExp {
    pub not: bool,
    pub arith_op: Box<ArtihOp>,
}

#[derive(Debug)]
pub enum ArtihOp {
    AddOp(AddOp),
    SubOp(SubOp),
    Relation(Relation),
}

#[derive(Debug)]
pub struct AddOp {
    pub arith_op: Box<ArtihOp>,
    pub relation: Box<Relation>,
}

#[derive(Debug)]
pub struct SubOp {
    pub arith_oo: Box<ArtihOp>,
    pub relation: Box<Relation>,
}

#[derive(Debug)]
pub enum Relation {
    LessThan(LessThan),
    LessThanEq(LessThanEq),
    GreaterThan(GreaterThan),
    GreaterThanEq(GreaterThanEq),
    Equals(Equals),
    NotEquals(NotEquals),
    Term,
}

#[derive(Debug)]
pub struct LessThan {
    pub relation: Box<Relation>,
    pub term: Term,
}

#[derive(Debug)]
pub struct LessThanEq {
    pub relation: Box<Relation>,
    pub term: Term,
}

#[derive(Debug)]
pub struct GreaterThan {
    pub relation: Box<Relation>,
    pub term: Term,
}

#[derive(Debug)]
pub struct GreaterThanEq {
    pub relation: Box<Relation>,
    pub term: Term,
}

#[derive(Debug)]
pub struct Equals {
    pub relation: Box<Relation>,
    pub term: Term,
}

#[derive(Debug)]
pub struct NotEquals {
    pub relation: Box<Relation>,
    pub term: Term,
}

#[derive(Debug)]
pub enum Term {
    MultTerm(MultTerm),
    DivTerm(DivTerm),
    Factor(Factor),
}

#[derive(Debug)]
pub struct MultTerm {
    pub term: Box<Term>,
    pub factor: Factor,
}

#[derive(Debug)]
pub struct DivTerm {
    pub term: Box<Term>,
    pub factor: Factor,
}

#[derive(Debug)]
pub enum Factor {
    Expression(Expression),
    ProcedureCall(ProcedureCall),
    Name { negate: bool, name: Name },
    Number { negate: bool, number: Number },
    String(StringNode),
    TrueLit,
    FalseLit,
}

#[derive(Debug)]
pub struct Name {
    pub identifier: Identifier,
    pub expression: Option<Expression>,
}
