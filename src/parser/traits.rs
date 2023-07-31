use super::utils::{ParserError, TokenQueue};

pub trait ParseTokens: Sized {
    fn parse(tokens: &mut TokenQueue) -> Result<Self, ParserError>;
}

pub trait CanParse {
    fn can_parse(tokens: &mut TokenQueue) -> bool;
}
