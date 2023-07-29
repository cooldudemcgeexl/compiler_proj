use super::utils::*;
use super::{declaratons::Declaration, statement::Statement};
use crate::tokens::Token;

pub struct ProgramStruct {
    program_header: ProgramHeader,
    program_body: ProgramBody,
}

impl ParseTokens for ProgramStruct {
    fn parse(tokens: &mut TokenQueue) -> Result<Self, ParserError> {
        let header = ProgramHeader::parse(tokens)?;
        let body = ProgramBody::parse(tokens)?;

        tokens.consume_expected(Token::Period)?;
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

pub struct ProgramHeader {
    header_identifier: String,
}

impl ParseTokens for ProgramHeader {
    fn parse(tokens: &mut TokenQueue) -> Result<Self, ParserError> {
        todo!()
    }
}

pub struct ProgramBody {
    declarations: Vec<Declaration>,
    statements: Vec<Statement>,
}

impl ParseTokens for ProgramBody {
    fn parse(tokens: &mut TokenQueue) -> Result<Self, ParserError> {
        todo!()
    }
}
