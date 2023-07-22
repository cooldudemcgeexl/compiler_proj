use std::ops::RangeInclusive;

#[derive(Debug)]
pub enum TokenType {
    PLUS,
    MINUS,
    IF_RW,
    LOOP_RW,
    END_RW,
    L_PAREN,
    IDENTIFIER{token_str: String},
    EOF
}

pub struct Token {
    token_type: TokenType
}

impl Token {
    pub fn set_token_type(token_type: TokenType) {
        todo!()
    }
}