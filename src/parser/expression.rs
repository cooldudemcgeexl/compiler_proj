use super::procedure::ProcedureCall;
use super::traits::ParseTokens;
use super::types::*;
use super::utils::{ParserError, TokenQueue};
use crate::tokens::Token;

#[derive(Debug)]
pub enum Expression {
    AndExp(Box<Expression>, ArtihOp),
    OrExp(Box<Expression>, ArtihOp),
    NotExp(ArtihOp),
    BasicExp(ArtihOp),
}

impl ParseTokens for Expression {
    fn parse(tokens: &mut TokenQueue) -> Result<Self, ParserError> {
        let mut expression = if tokens.consume_as_bool(&Token::Not) {
            Expression::NotExp(ArtihOp::parse(tokens)?)
        } else {
            Expression::BasicExp(ArtihOp::parse(tokens)?)
        };

        loop {
            expression = match tokens.peek_front() {
                Some(Token::Amp) => {
                    tokens.pop_front();
                    let arith_op = ArtihOp::parse(tokens)?;
                    Expression::AndExp(Box::new(expression), arith_op)
                }
                Some(Token::Pipe) => {
                    tokens.pop_front();
                    let arith_op = ArtihOp::parse(tokens)?;
                    Expression::OrExp(Box::new(expression), arith_op)
                }
                _ => return Ok(expression),
            }
        }
    }
}

#[derive(Debug)]
pub enum ArtihOp {
    AddOp(Box<ArtihOp>, Relation),
    SubOp(Box<ArtihOp>, Relation),
    Relation(Relation),
}

impl ParseTokens for ArtihOp {
    fn parse(tokens: &mut TokenQueue) -> Result<Self, ParserError> {
        let mut arith_op = ArtihOp::Relation(Relation::parse(tokens)?);

        loop {
            arith_op = match tokens.peek_front() {
                Some(Token::Plus) => {
                    tokens.pop_front();
                    let next_relation = Relation::parse(tokens)?;
                    ArtihOp::AddOp(Box::new(arith_op), next_relation)
                }
                Some(Token::Minus) => {
                    tokens.pop_front();
                    let next_relation = Relation::parse(tokens)?;
                    ArtihOp::SubOp(Box::new(arith_op), next_relation)
                }
                _ => return Ok(arith_op),
            }
        }
    }
}

#[derive(Debug)]
pub enum Relation {
    LessThan(Box<Relation>, Term),
    LessThanEq(Box<Relation>, Term),
    GreaterThan(Box<Relation>, Term),
    GreaterThanEq(Box<Relation>, Term),
    Equals(Box<Relation>, Term),
    NotEquals(Box<Relation>, Term),
    Term(Term),
}

impl ParseTokens for Relation {
    fn parse(tokens: &mut TokenQueue) -> Result<Self, ParserError> {
        let mut relation = Relation::Term(Term::parse(tokens)?);

        loop {
            relation = match tokens.peek_front() {
                Some(Token::LessThan) => {
                    tokens.pop_front();
                    let next_term = Term::parse(tokens)?;
                    Relation::LessThan(Box::new(relation), next_term)
                }
                Some(Token::LessThanEq) => {
                    tokens.pop_front();
                    let next_term = Term::parse(tokens)?;
                    Relation::LessThanEq(Box::new(relation), next_term)
                }
                Some(Token::GreaterThan) => {
                    tokens.pop_front();
                    let next_term = Term::parse(tokens)?;
                    Relation::GreaterThan(Box::new(relation), next_term)
                }
                Some(Token::GreaterThanEq) => {
                    tokens.pop_front();
                    let next_term = Term::parse(tokens)?;
                    Relation::GreaterThanEq(Box::new(relation), next_term)
                }
                Some(Token::EqualsComp) => {
                    tokens.pop_front();
                    let next_term = Term::parse(tokens)?;
                    Relation::Equals(Box::new(relation), next_term)
                }
                Some(Token::NotEquals) => {
                    tokens.pop_front();
                    let next_term = Term::parse(tokens)?;
                    Relation::NotEquals(Box::new(relation), next_term)
                }
                _ => return Ok(relation),
            }
        }
    }
}

#[derive(Debug)]
pub enum Term {
    MultTerm(Box<Term>, Factor),
    DivTerm(Box<Term>, Factor),
    Factor(Factor),
}

impl ParseTokens for Term {
    fn parse(tokens: &mut TokenQueue) -> Result<Self, ParserError> {
        let mut term = Term::Factor(Factor::parse(tokens)?);

        loop {
            term = match tokens.peek_front() {
                Some(Token::Mult) => {
                    tokens.pop_front();
                    let next_factor = Factor::parse(tokens)?;
                    Term::MultTerm(Box::new(term), next_factor)
                }
                Some(Token::Div) => {
                    tokens.pop_front();
                    let next_factor = Factor::parse(tokens)?;
                    Term::DivTerm(Box::new(term), next_factor)
                }
                _ => return Ok(term),
            }
        }
    }
}

#[derive(Debug)]
pub enum Factor {
    NestedExpression(Box<Expression>),
    ProcedureCall(ProcedureCall),
    Name { negate: bool, name: Name },
    Number { negate: bool, number: Number },
    String(StringNode),
    TrueLit,
    FalseLit,
}

impl ParseTokens for Factor {
    fn parse(tokens: &mut TokenQueue) -> Result<Self, ParserError> {
        match tokens.pop_front() {
            Some(Token::LParen) => {
                let expression = Expression::parse(tokens)?;
                tokens.consume_expected(Token::RParen)?;
                Ok(Factor::NestedExpression(Box::new(expression)))
            }
            Some(Token::True) => Ok(Factor::TrueLit),
            Some(Token::False) => Ok(Factor::FalseLit),
            Some(Token::StringLiteral(value)) => Ok(Factor::String(StringNode {
                literal_string: value,
            })),

            Some(Token::Minus) => match tokens.peek_front() {
                Some(Token::Identifier(_)) => Ok(Factor::Name {
                    negate: true,
                    name: Name::parse(tokens)?,
                }),
                Some(Token::NumberLiteral(_)) => Ok(Factor::Number {
                    negate: true,
                    number: Number::parse(tokens)?,
                }),
                Some(token) => Err(ParserError::UnexpectedToken(
                    String::from("Negation"),
                    token.clone(),
                )),
                None => Err(ParserError::UnexpectedEOF(String::from("Negation"))),
            },
            Some(Token::Identifier(value)) => match tokens.peek_front() {
                Some(Token::LParen) => {
                    tokens.push_front(Token::Identifier(value));
                    Ok(Factor::ProcedureCall(ProcedureCall::parse(tokens)?))
                }
                _ => {
                    tokens.push_front(Token::Identifier(value));
                    Ok(Factor::Name {
                        negate: false,
                        name: Name::parse(tokens)?,
                    })
                }
            },
            Some(Token::NumberLiteral(value)) => {
                tokens.push_front(Token::NumberLiteral(value));
                Ok(Factor::Number {
                    negate: false,
                    number: Number::parse(tokens)?,
                })
            }
            Some(token) => Err(ParserError::UnexpectedToken(String::from("Factor"), token)),
            None => Err(ParserError::UnexpectedEOF(String::from("Factor"))),
        }
    }
}

#[derive(Debug)]
pub struct Name {
    pub identifier: Identifier,
    pub expression: Option<Box<Expression>>,
}

impl ParseTokens for Name {
    fn parse(tokens: &mut TokenQueue) -> Result<Self, ParserError> {
        let identifier = tokens.consume_identifier()?;
        if tokens.consume_as_bool(&Token::LBracket) {
            let expression = Expression::parse(tokens)?;
            tokens.consume_expected(Token::RBracket)?;
            Ok(Name {
                identifier: Identifier {
                    identifier_string: identifier,
                },
                expression: Some(Box::new(expression)),
            })
        } else {
            Ok(Name {
                identifier: Identifier {
                    identifier_string: identifier,
                },
                expression: None,
            })
        }
    }
}
