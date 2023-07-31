use super::expression::Expression;
use super::traits::{CanParse, ParseTokens};
use super::types::Identifier;
use super::utils::{ParserError, TokenQueue};
use crate::tokens::Token;

#[derive(Debug)]
pub enum Statement {
    Assignment(AssignmentStatement),
    If(IfStatement),
    Loop(LoopStatement),
    Return(ReturnStatement),
}

impl ParseTokens for Statement {
    fn parse(tokens: &mut TokenQueue) -> Result<Self, ParserError> {
        match tokens.peek_front() {
            Some(Token::Identifier(_)) => {
                Ok(Statement::Assignment(AssignmentStatement::parse(tokens)?))
            }
            Some(Token::If) => Ok(Statement::If(IfStatement::parse(tokens)?)),
            Some(Token::For) => Ok(Statement::Loop(LoopStatement::parse(tokens)?)),
            Some(Token::Return) => Ok(Statement::Return(ReturnStatement::parse(tokens)?)),
            Some(token) => Err(ParserError::UnexpectedToken(
                String::from("Satement"),
                token.clone(),
            )),
            None => Err(ParserError::UnexpectedEOF(String::from("Statement"))),
        }
    }
}

impl CanParse for Statement {
    fn can_parse(tokens: &mut TokenQueue) -> bool {
        match tokens.peek_front() {
            Some(Token::If) => true,
            Some(Token::For) => true,
            Some(Token::Return) => true,
            Some(Token::Identifier(_)) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct AssignmentStatement {
    pub destination: Destination,
    pub expression: Expression,
}

impl ParseTokens for AssignmentStatement {
    fn parse(tokens: &mut TokenQueue) -> Result<Self, ParserError> {
        let destination = Destination::parse(tokens)?;
        tokens.consume_expected(Token::Assignment)?;
        let expression = Expression::parse(tokens)?;
        Ok(AssignmentStatement {
            destination: destination,
            expression: expression,
        })
    }
}

#[derive(Debug)]
pub struct IfStatement {
    pub condition: Expression,
    pub then_statement: Vec<Statement>,
    pub else_statement: Option<Vec<Statement>>,
}

impl ParseTokens for IfStatement {
    fn parse(tokens: &mut TokenQueue) -> Result<Self, ParserError> {
        let mut then_block = Vec::new();
        let mut else_block = Vec::new();

        // Take care of "header"
        tokens.consume_expected(Token::If)?;
        tokens.consume_expected(Token::LParen)?;
        let condition = Expression::parse(tokens)?;
        tokens.consume_expected(Token::RParen)?;
        tokens.consume_expected(Token::Then)?;

        // Then block
        while Statement::can_parse(tokens) {
            then_block.push(Statement::parse(tokens)?);
            tokens.consume_expected(Token::Semicolon)?;
        }

        // Check if else block exists
        let else_exists = tokens.consume_as_bool(&Token::Else);

        // Else block
        if else_exists {
            tokens.pop_front();
            while Statement::can_parse(tokens) {
                else_block.push(Statement::parse(tokens)?);
                tokens.consume_expected(Token::Semicolon)?;
            }
        }

        // End If
        tokens.consume_expected(Token::End)?;
        tokens.consume_expected(Token::If)?;

        Ok(IfStatement {
            condition,
            then_statement: then_block,
            else_statement: if else_exists { Some(else_block) } else { None },
        })
    }
}

#[derive(Debug)]
pub struct LoopStatement {
    pub assignment_statement: AssignmentStatement,
    pub condition: Expression,
    pub loop_body: Vec<Statement>,
}

impl ParseTokens for LoopStatement {
    fn parse(tokens: &mut TokenQueue) -> Result<Self, ParserError> {
        let mut loop_body = Vec::new();

        tokens.consume_expected(Token::For)?;
        tokens.consume_expected(Token::LParen)?;

        let assignment_statement = AssignmentStatement::parse(tokens)?;
        tokens.consume_expected(Token::Semicolon)?;

        let condition = Expression::parse(tokens)?;
        tokens.consume_expected(Token::RParen)?;

        // Loop body
        while Statement::can_parse(tokens) {
            loop_body.push(Statement::parse(tokens)?);
            tokens.consume_expected(Token::Semicolon)?;
        }

        tokens.consume_expected(Token::End)?;
        tokens.consume_expected(Token::For)?;

        Ok(LoopStatement {
            assignment_statement: assignment_statement,
            condition,
            loop_body,
        })
    }
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub expression: Expression,
}

impl ParseTokens for ReturnStatement {
    fn parse(tokens: &mut TokenQueue) -> Result<Self, ParserError> {
        tokens.consume_expected(Token::Return)?;

        let expression = Expression::parse(tokens)?;

        Ok(ReturnStatement {
            expression: expression,
        })
    }
}

#[derive(Debug)]
pub struct Destination {
    pub identifier: Identifier,
    pub expression: Option<Expression>,
}

impl ParseTokens for Destination {
    fn parse(tokens: &mut TokenQueue) -> Result<Self, ParserError> {
        let identifier = tokens.consume_identifier()?;

        if tokens.consume_as_bool(&Token::LBracket) {
            let expression = Expression::parse(tokens)?;
            tokens.consume_expected(Token::RBracket)?;
            Ok(Destination {
                identifier: Identifier {
                    identifier_string: identifier,
                },
                expression: Some(expression),
            })
        } else {
            Ok(Destination {
                identifier: Identifier {
                    identifier_string: identifier,
                },
                expression: None,
            })
        }
    }
}
