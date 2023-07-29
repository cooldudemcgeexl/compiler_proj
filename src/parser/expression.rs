use super::procedure::ProcedureCall;
use super::types::*;

pub enum Expression {
    AndExp(AndExpression),
    OrExp(OrExpression),
    BasicExp(BasicExp),
}

pub struct AndExpression {
    pub expression: Box<Expression>,
    pub arith_op: Box<ArtihOp>,
}

pub struct OrExpression {
    pub expression: Box<Expression>,
    pub arith_op: Box<ArtihOp>,
}

pub struct BasicExp {
    pub not: bool,
    pub arith_op: Box<ArtihOp>,
}

pub enum ArtihOp {
    AddOp(AddOp),
    SubOp(SubOp),
    Relation(Relation),
}

pub struct AddOp {
    pub arith_op: Box<ArtihOp>,
    pub relation: Box<Relation>,
}

pub struct SubOp {
    pub arith_oo: Box<ArtihOp>,
    pub relation: Box<Relation>,
}

pub enum Relation {
    LessThan(LessThan),
    LessThanEq(LessThanEq),
    GreaterThan(GreaterThan),
    GreaterThanEq(GreaterThanEq),
    Equals(Equals),
    NotEquals(NotEquals),
    Term,
}

pub struct LessThan {
    pub relation: Box<Relation>,
    pub term: Term,
}

pub struct LessThanEq {
    pub relation: Box<Relation>,
    pub term: Term,
}

pub struct GreaterThan {
    pub relation: Box<Relation>,
    pub term: Term,
}

pub struct GreaterThanEq {
    pub relation: Box<Relation>,
    pub term: Term,
}

pub struct Equals {
    pub relation: Box<Relation>,
    pub term: Term,
}

pub struct NotEquals {
    pub relation: Box<Relation>,
    pub term: Term,
}

pub enum Term {
    MultTerm(MultTerm),
    DivTerm(DivTerm),
    Factor(Factor),
}

pub struct MultTerm {
    pub term: Box<Term>,
    pub factor: Factor,
}

pub struct DivTerm {
    pub term: Box<Term>,
    pub factor: Factor,
}

pub enum Factor {
    Expression(Expression),
    ProcedureCall(ProcedureCall),
    Name { negate: bool, name: Name },
    Number { negate: bool, number: Number },
    String(StringNode),
    TrueLit,
    FalseLit,
}

pub struct Name {
    pub identifier: Identifier,
    pub expression: Option<Expression>,
}
