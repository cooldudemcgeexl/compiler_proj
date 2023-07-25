
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
    GreaterThanEq,
    LessThan,
    LessThanEq,
    EqualsComp,
    NotEquals,
    LBracket,
    RBracket,
    LParen,
    RParen,
    Amp,
    Pipe,
    Colon,
    Period,
    Semicolon,

    // Identifiers
    Identifier(String),
    Function(String),
    NumberLiteral(String),
    StringLiteral(String),

    // Special :)
    EOF

}

impl Token {
    /// Returns the token for single character tokens
    pub fn from_char(symbol_char: char) -> Token {
        match symbol_char {
            '+' => Token::Plus,
            '-' => Token::Minus,
            _ => todo!()
        }
    }
}

#[derive(Debug)]
pub enum BuildToken {
    None,
    Symbol(String),
    Identifier(String)
}