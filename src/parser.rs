use crate::tokens::Token;
use std::collections::VecDeque;
use thiserror::Error;

use self::{
    program::ProgramStruct,
    utils::{ParseTokens, ParserError, TokenQueue},
};

mod declaratons;
mod expression;
mod procedure;
mod program;
mod statement;
mod types;
pub mod utils;

pub fn parse_tokens(token_deque: VecDeque<Token>) -> Result<ProgramStruct, ParserError> {
    let mut tokens = TokenQueue::new(token_deque);
    ProgramStruct::parse(&mut tokens)
}
