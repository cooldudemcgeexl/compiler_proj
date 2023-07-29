use crate::tokens::Token;
use std::collections::VecDeque;
use thiserror::Error;

mod declaratons;
mod expression;
mod procedure;
mod program;
mod statement;
mod types;
mod utils;

#[derive(Error, Debug)]
pub enum ParserError {}

pub fn parse_tokens(token_deque: VecDeque<Token>) {}
