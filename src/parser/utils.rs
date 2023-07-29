use std::collections::VecDeque;
use thiserror::Error;

use crate::tokens::Token;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Encountered EOF token before exhausting token queue.")]
    EarlyEOF,
    #[error("Encountered EOF token. Expected token: {0:?}")]
    UnexpectedEOF(Token),
    #[error("Encountered token: {0:?} Expected EOF.")],
    ExpectedEOF(Token),
    #[error("Encountered token: {0} Expected token: {1:?}")]
    UnexpectedToken(String, Token),
}
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

    pub fn front(&self) -> Option<&Token> {
        self.tokens.front()
    }

    pub fn remaining(&self) -> usize {
        self.tokens.len()
    }

    pub fn consume_expected(&mut self, expected: Token) -> Result<(), ParserError> {
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
        Err(ParserError::UnexpectedEOF(expected))
    }
}

pub trait ParseTokens: Sized {
    fn parse(tokens: &mut TokenQueue) -> Result<Self, ParserError>;
}

pub trait CanParse {
    fn can_parse(tokens: &mut TokenQueue) -> bool;
}
