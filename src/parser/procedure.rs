use crate::tokens::Token;

use super::declaratons::{self, Declaration, VariableDeclaration};
use super::expression::Expression;
use super::statement::Statement;
use super::traits::ParseTokens;
use super::types::{Identifier, TypeMark};
use super::utils::{ParserError, TokenQueue};
#[derive(Debug)]
pub struct ProcedureHeader {
    pub identifier: String,
    pub type_mark: TypeMark,
    pub param_list: Option<ParamList>,
}

impl ParseTokens for ProcedureHeader {
    fn parse(tokens: &mut TokenQueue) -> Result<Self, ParserError> {
        tokens.consume_expected(Token::Procedure)?;
        let identifier = tokens.consume_identifier()?;
        tokens.consume_expected(Token::Colon)?;
        let type_mark = TypeMark::parse(tokens)?;
        tokens.consume_expected(Token::LParen)?;

        match tokens.peek_front() {
            Some(Token::RParen) => {
                tokens.pop_front();
                Ok(ProcedureHeader {
                    identifier: identifier,
                    type_mark: type_mark,
                    param_list: None,
                })
            }
            Some(Token::Variable) => {
                let params = ParamList::parse(tokens)?;
                tokens.consume_expected(Token::RParen)?;
                Ok(ProcedureHeader {
                    identifier: identifier,
                    type_mark: type_mark,
                    param_list: Some(params),
                })
            }
            Some(token) => Err(ParserError::UnexpectedToken(
                String::from("RParen,VariableDeclaration"),
                token.clone(),
            )),
            None => Err(ParserError::UnexpectedEOF(String::from(
                "RParen,VariableDeclaration",
            ))),
        }
    }
}

#[derive(Debug)]
pub struct ProcedureBody {
    pub declarations: Vec<Declaration>,
    pub statements: Vec<Statement>,
}

impl ParseTokens for ProcedureBody {
    fn parse(tokens: &mut TokenQueue) -> Result<Self, ParserError> {
        let mut declaratons = Vec::new();
        let mut statements = Vec::new();

        loop {
            let next_token = tokens.peek_front();
            if let Some(Token::Begin) = next_token {
                break; // Next token is Begin. We're at the end of the declarations block.
            } else if let None = next_token {
                return Err(ParserError::UnexpectedEOF(String::from(
                    "Identifier, Begin",
                )));
            } else {
                declaratons.push(Declaration::parse(tokens)?);
                tokens.consume_expected(Token::Semicolon)?;
            }
        }
        tokens.consume_expected(Token::Begin)?; // Start Statements after this

        loop {
            let next_token = tokens.peek_front();
            if let Some(Token::End) = next_token {
                break; // Next token is End. We're at the end of the statements block.
            } else if let None = next_token {
                return Err(ParserError::UnexpectedEOF(String::from("Identifier, End")));
            } else {
                statements.push(Statement::parse(tokens)?);
                tokens.consume_expected(Token::Semicolon)?;
            }
        }
        tokens.consume_expected(Token::End)?;
        tokens.consume_expected(Token::Procedure)?;

        Ok(ProcedureBody {
            declarations: declaratons,
            statements: statements,
        })
    }
}
#[derive(Debug)]
pub struct ParamList {
    pub param_list: Vec<Parameter>,
}

impl ParseTokens for ParamList {
    fn parse(tokens: &mut TokenQueue) -> Result<Self, ParserError> {
        let mut parameters = Vec::new();

        parameters.push(Parameter::parse(tokens)?);

        while let Some(&Token::Comma) = tokens.peek_front() {
            tokens.consume_expected(Token::Comma)?;
            parameters.push(Parameter::parse(tokens)?);
        }

        Ok(ParamList {
            param_list: parameters,
        })
    }
}

#[derive(Debug)]
pub struct Parameter {
    pub variable_declaration: VariableDeclaration,
}

impl ParseTokens for Parameter {
    fn parse(tokens: &mut TokenQueue) -> Result<Self, ParserError> {
        VariableDeclaration::parse(tokens).map(|token| Parameter {
            variable_declaration: token,
        })
    }
}

#[derive(Debug)]
pub struct ProcedureCall {
    pub identifier: Identifier,
    pub arg_list: Option<ArgumentList>,
}

impl ParseTokens for ProcedureCall {
    fn parse(tokens: &mut TokenQueue) -> Result<Self, ParserError> {
        let identifier = tokens.consume_identifier()?;
        tokens.consume_expected(Token::LParen)?;

        match tokens.peek_front() {
            Some(Token::RParen) => {
                tokens.pop_front();
                Ok(ProcedureCall {
                    identifier: Identifier {
                        identifier_string: identifier,
                    },
                    arg_list: None,
                })
            }
            Some(_) => {
                let args = ArgumentList::parse(tokens)?;
                tokens.consume_expected(Token::RParen)?;
                Ok(ProcedureCall {
                    identifier: Identifier {
                        identifier_string: identifier,
                    },
                    arg_list: Some(args),
                })
            }
            None => Err(ParserError::UnexpectedEOF(String::from(
                "RParen,ArgumentList",
            ))),
        }
    }
}

#[derive(Debug)]
pub struct ArgumentList {
    pub expr_list: Vec<Expression>,
}

impl ParseTokens for ArgumentList {
    fn parse(tokens: &mut TokenQueue) -> Result<Self, ParserError> {
        let mut args = Vec::new();

        args.push(Expression::parse(tokens)?);

        while let Some(&Token::Comma) = tokens.peek_front() {
            tokens.consume_expected(Token::Comma)?;
            args.push(Expression::parse(tokens)?);
        }

        Ok(ArgumentList { expr_list: args })
    }
}
