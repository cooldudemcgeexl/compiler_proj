use crate::tokens::Token;
use std::collections::VecDeque;
use thiserror::Error;

use self::{
    program::ProgramStruct,
    traits::ParseTokens,
    utils::{ParserError, TokenQueue},
};

pub mod declaratons;
pub mod expression;
pub mod procedure;
pub mod program;
pub mod statement;
pub mod traits;
pub mod types;
pub mod utils;

pub fn parse_tokens(token_deque: VecDeque<Token>) -> Result<ProgramStruct, ParserError> {
    let mut tokens = TokenQueue::new(token_deque);
    ProgramStruct::parse(&mut tokens)
}
