use thiserror::Error;

#[derive(Debug)]
/// Terminals from the EBNF Grammar Provided
pub enum Token {
    // Keywords
    Program,
    Is,
    Begin,
    End,
    Global,
    Procedure,
    Variable,
    For,
    Not,
    If,
    Then,
    Else,

    // Types
    Integer,
    Float,
    String,
    Bool,

    // Symbols
    Plus,
    Minus,
    Mult,
    Div,
    GreaterThan,
    LessThan,
    LBracket,
    RBracket,
    LParen,
    RParen,
    Amp,
    Pipe,
    Colon,
    Period,
    Semicolon,
    Comma,

    // Symbol Pairs
    GreaterThanEq,
    LessThanEq,
    EqualsComp,
    NotEquals,
    Assignment,

    // Identifiers
    Identifier(String),
    Function(String),
    NumberLiteral(String),
    StringLiteral(String),

    // Special :)
    EOF,
}

impl Token {
    /// Returns the token for single character tokens
    pub fn from_char(symbol_char: char) -> Token {
        match symbol_char {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Mult,
            '/' => Token::Div,
            '<' => Token::LessThan,
            '>' => Token::GreaterThan,
            '[' => Token::LBracket,
            ']' => Token::RBracket,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '&' => Token::Amp,
            '|' => Token::Pipe,
            ':' => Token::Colon,
            '.' => Token::Period,
            ';' => Token::Semicolon,
            ',' => Token::Comma,
            _ => todo!(),
        }
    }

    pub fn from_compound_identifier(compound_chars: &str) -> Token {
        match compound_chars {
            ":=" => Token::Assignment,
            "==" => Token::EqualsComp,
            "!=" => Token::NotEquals,
            "<=" => Token::LessThanEq,
            ">=" => Token::GreaterThanEq,
            _ => todo!()
        }
    }
}

#[derive(Error, Debug)]
enum TokenError {}

#[derive(Debug)]
pub enum BuildToken {
    /// Not building a token
    None,
    /// Building a multicharacter symbol (i.e. !=. ==, >=)
    CompoundSymbol(String),
    Identifier(String),
}
