use crate::tokens::Token;
use std::collections::VecDeque;

mod declaratons;
mod expression;
mod procedure;
mod program;
mod statement;
mod types;

pub fn parse_tokens(token_deque: VecDeque<Token>) {}
