pub mod stripper;


use thiserror::Error;
use crate::tokens::{BuildToken, Token};

#[derive(Error, Debug)]
pub enum ScannerError {
    #[error("Comment stripper encountered error")]
    StripError(#[from] stripper::StripError),
}



const SINGLE_CHARS: &str = "+-*/[]()&|.;,";
const POSSIBLE_COMPOUNDS: &str = "<>=:!";

pub fn scan(file_contents: String) -> Result<Vec<Token>, ScannerError> {
    let mut line_number = 0u32;
    let mut token_vec: Vec<Token> = vec![];
    let cleaned_file = stripper::strip_comments(file_contents)?;
    let mut current_token = BuildToken::None;
    for (index, curr_char) in cleaned_file.chars().enumerate() {
        current_token = match (curr_char, current_token) {
            (' ' | '\t' | '\n', BuildToken::None) => BuildToken::None,
            (curr_char, BuildToken::None) if SINGLE_CHARS.contains(curr_char) => {
                token_vec.push(Token::from_char(curr_char));
                BuildToken::None
            }
            (curr_char, BuildToken::None) if POSSIBLE_COMPOUNDS.contains(curr_char) => {
                BuildToken::CompoundSymbol(String::from(curr_char))
            }

            ('=', BuildToken::CompoundSymbol(string)) => {
                let compound_chars = format!("{string}=");
                token_vec.push(Token::from_compound_identifier(compound_chars.as_str()));
                BuildToken::None
            }
            (' ' | '\t' | '\n', BuildToken::CompoundSymbol(string)) => {
                let string_char = string.chars().next().unwrap();
                token_vec.push(Token::from_char(string_char));
                BuildToken::None
            }

            ('0'..='9', BuildToken::None) => BuildToken::NumberLiteral(String::from(curr_char)),
            ('0'..='9' | '_', BuildToken::NumberLiteral(string)) => {
                let updated_literal = format!("{string}{curr_char}");
                BuildToken::NumberLiteral(updated_literal)
            }
            ('.', BuildToken::NumberLiteral(string)) if !string.contains('.') => {
                let updated_literal = format!("{string}{curr_char}");
                BuildToken::NumberLiteral(updated_literal)
            }
            ('.', BuildToken::NumberLiteral(string)) => {
                todo!()
            }

            (curr_char, BuildToken::NumberLiteral(string)) if SINGLE_CHARS.contains(curr_char) => {
                token_vec.push(Token::num_literal_from_string(string));
                token_vec.push(Token::from_char(curr_char));
                BuildToken::None
            }
            (curr_char, BuildToken::NumberLiteral(string))
                if POSSIBLE_COMPOUNDS.contains(curr_char) =>
            {
                token_vec.push(Token::num_literal_from_string(string));
                BuildToken::CompoundSymbol(String::from(curr_char))
            }
            (' ' | '\t' | '\n', BuildToken::NumberLiteral(string)) => {
                token_vec.push(Token::num_literal_from_string(string));
                BuildToken::None
            }

            ('"', BuildToken::None) => BuildToken::StringLiteral(String::from("")),
            ('"', BuildToken::StringLiteral(string)) => {
                token_vec.push(Token::string_literal_from_string(string));
                BuildToken::None
            }
            (_, BuildToken::StringLiteral(string)) => {
                BuildToken::StringLiteral(format!("{string}{curr_char}"))
            }

            ('a'..='z' | 'A'..='Z', BuildToken::None) => {
                BuildToken::Identifier(String::from(curr_char))
            }
            ('a'..='z' | 'A'..='Z' | '0'..='9' | '_', BuildToken::Identifier(string)) => {
                BuildToken::Identifier(format!("{string}{curr_char}"))
            }
            (curr_char, BuildToken::Identifier(string)) if SINGLE_CHARS.contains(curr_char) => {
                token_vec.push(Token::from_string(string));
                token_vec.push(Token::from_char(curr_char));
                BuildToken::None
            }
            (curr_char, BuildToken::Identifier(string))
                if POSSIBLE_COMPOUNDS.contains(curr_char) =>
            {
                token_vec.push(Token::Identifier(string));
                BuildToken::CompoundSymbol(String::from(curr_char))
            }
            (' ' | '\t' | '\n', BuildToken::Identifier(string)) => {
                token_vec.push(Token::from_string(string));
                BuildToken::None
            }

            _ => BuildToken::None,
        }
    }
    token_vec.push(Token::EOF);
    Ok(token_vec)
}

#[cfg(test)]
use rstest::rstest;
use std::fs;
use std::path::PathBuf;
#[cfg(test)]
#[rstest]
fn test_scan(#[files("tests/correct/*.src")] path: PathBuf) {
    let res_path = path.as_path();
    let file_name = path.file_name().unwrap().to_str().unwrap();
    let test_file_text = fs::read_to_string(res_path).unwrap();
    let token_vec = scan(test_file_text).unwrap();
    println!("{:?}: {:?}\n", file_name, token_vec);
}
