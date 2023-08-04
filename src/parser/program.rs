use super::traits::ParseTokens;
use super::utils::*;
use super::{declaratons::Declaration, statement::Statement};
use crate::tokens::Token;

#[derive(Debug)]
pub struct ProgramStruct {
    pub program_header: ProgramHeader,
    pub program_body: ProgramBody,
}

impl ParseTokens for ProgramStruct {
    fn parse(tokens: &mut TokenQueue) -> Result<Self, ParserError> {
        let header = ProgramHeader::parse(tokens)?;
        let body = ProgramBody::parse(tokens)?;

        tokens
            .consume_expected(Token::Period)
            .unwrap_or_else(|_| println!("WARNING: Missing terminating period"));
        tokens.consume_expected(Token::EOF)?;

        if let Some(next_token) = tokens.pop_front() {
            return Err(ParserError::ExpectedEOF(next_token));
        }

        Ok(ProgramStruct {
            program_header: header,
            program_body: body,
        })
    }
}

#[derive(Debug)]
pub struct ProgramHeader {
    pub header_identifier: String,
}

impl ParseTokens for ProgramHeader {
    fn parse(tokens: &mut TokenQueue) -> Result<Self, ParserError> {
        tokens.consume_expected(Token::Program)?;
        let header_identifier = tokens.consume_identifier()?;
        tokens.consume_expected(Token::Is)?;

        Ok(ProgramHeader { header_identifier })
    }
}

#[derive(Debug)]
pub struct ProgramBody {
    pub declarations: Vec<Declaration>,
    pub statements: Vec<Statement>,
}

impl ParseTokens for ProgramBody {
    fn parse(tokens: &mut TokenQueue) -> Result<Self, ParserError> {
        let mut declaratons: Vec<Declaration> = Vec::<Declaration>::new();
        let mut statements: Vec<Statement> = Vec::<Statement>::new();

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
        tokens.consume_expected(Token::Program)?;

        Ok(ProgramBody {
            declarations: declaratons,
            statements: statements,
        })
    }
}
