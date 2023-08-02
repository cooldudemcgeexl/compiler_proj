use std::collections::VecDeque;
use thiserror::Error;

use crate::tokens::Token;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Encountered EOF token before exhausting token queue.")]
    EarlyEOF,
    #[error("Encountered EOF token. Expected token: {0:?}")]
    UnexpectedEOFToken(Token),
    #[error("Encountered EOF. Expected the following tokens: {0}")]
    UnexpectedEOF(String),
    #[error("Encountered token: {0:?} Expected EOF.")]
    ExpectedEOF(Token),
    #[error("Expected token: {0} Encountered token: {1:?}")]
    UnexpectedToken(String, Token),
}
#[derive(Debug)]
pub struct TokenQueue {
    tokens: VecDeque<Token>,
}

impl TokenQueue {
    pub fn new(tokens: VecDeque<Token>) -> Self {
        TokenQueue { tokens: tokens }
    }

    pub fn pop_front(&mut self) -> Option<Token> {
        self.tokens.pop_front()
    }

    pub fn push_front(&mut self, value: Token) -> () {
        self.tokens.push_front(value)
    }

    pub fn peek_front(&self) -> Option<&Token> {
        self.tokens.front()
    }

    pub fn remaining(&self) -> usize {
        self.tokens.len()
    }

    pub fn consume_expected(&mut self, expected: Token) -> Result<(), ParserError> {
        if self.remaining() == 0 && expected == Token::EOF {
            return Ok(());
        }
        if let Some(token) = self.pop_front() {
            if self.remaining() > 0 && token == Token::EOF {
                return Err(ParserError::EarlyEOF);
            }
            if token != expected {
                return Err(ParserError::UnexpectedToken(
                    format!("{:?}", expected),
                    token,
                ));
            }
            return Ok(());
        }
        Err(ParserError::UnexpectedEOFToken(expected))
    }

    /// Need a separate function for consuming identifiers, since we need to take the data out of them.
    pub fn consume_identifier(&mut self) -> Result<String, ParserError> {
        match self.pop_front() {
            Some(Token::Identifier(ident)) => Ok(ident),
            Some(token) => Err(ParserError::UnexpectedToken(
                String::from("Identifier"),
                token,
            )),
            _ => Err(ParserError::UnexpectedEOFToken(Token::Identifier(
                String::from("Identifier"),
            ))),
        }
    }

    /// Peeks the next token. If it matches, consumes the token and returns true.
    /// Otherwise, false.
    pub fn consume_as_bool(&mut self, expected: &Token) -> bool {
        match self.peek_front() {
            Some(token) if token == expected => {
                self.pop_front();
                true
            }
            _ => false,
        }
    }
}
