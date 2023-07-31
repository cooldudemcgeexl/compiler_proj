use super::procedure::{ProcedureBody, ProcedureHeader};
use super::traits::ParseTokens;
use super::types::{ArrayBound, Identifier, TypeMark};
use super::utils::ParserError;
use crate::tokens::Token;

#[derive(Debug)]
pub enum Declaration {
    Procedure(bool, ProcedureDeclaration),
    Variable(bool, VariableDeclaration),
}

impl ParseTokens for Declaration {
    fn parse(tokens: &mut super::utils::TokenQueue) -> Result<Self, super::utils::ParserError> {
        let is_global = tokens.consume_as_bool(&Token::Global);
        match tokens.peek_front() {
            Some(Token::Procedure) => Ok(Declaration::Procedure(is_global, todo!())),
            Some(Token::Variable) => Ok(Declaration::Variable(is_global, todo!())),
            Some(token) => Err(ParserError::UnexpectedToken(
                String::from("Declaration"),
                token.clone(),
            )),
            None => Err(ParserError::UnexpectedEOF(String::from("Declaration"))),
        }
    }
}
#[derive(Debug)]
pub struct ProcedureDeclaration {
    pub procedure_header: ProcedureHeader,
    pub procedure_body: ProcedureBody,
}

impl ParseTokens for ProcedureDeclaration {
    fn parse(tokens: &mut super::utils::TokenQueue) -> Result<Self, ParserError> {
        let proc_header = todo!();
        let proc_body = todo!();
        Ok(ProcedureDeclaration {
            procedure_header: proc_header,
            procedure_body: proc_body,
        })
    }
}
#[derive(Debug)]
pub struct VariableDeclaration {
    pub identifier: String,
    pub type_mark: TypeMark,
    pub array_bound: Option<ArrayBound>,
}

impl ParseTokens for VariableDeclaration {
    fn parse(tokens: &mut super::utils::TokenQueue) -> Result<Self, ParserError> {
        tokens.consume_expected(Token::Variable)?;
        let Identifier = tokens.consume_identifier()?;
        tokens.consume_expected(Token::Colon)?;
        let type_mark = TypeMark::parse(tokens)?;

        let is_bounded = tokens.consume_as_bool(&Token::LBracket);
        let array_bound = if is_bounded {
            let array_bound = ArrayBound::parse(tokens)?;
            tokens.consume_expected(Token::RBracket)?;
            Some(array_bound)
        } else {
            None
        };

        Ok(VariableDeclaration {
            identifier: Identifier,
            type_mark: type_mark,
            array_bound: array_bound,
        })
    }
}
