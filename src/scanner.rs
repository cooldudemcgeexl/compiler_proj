
pub mod scanner_file;
pub mod tokens;
pub mod stripper;

use thiserror::Error;
use tokens::{Token, BuildToken};

#[derive(Error, Debug)]
pub enum ScannerError {
    #[error("Comment stripper encountered error")]
    StripError(#[from] stripper::StripError)

}

enum ScanState {
    Normal,

}

pub fn scan(file_contents: String) -> Result<(),ScannerError> {
    let mut line_number = 0u32;
    let mut token_vec: Vec<Token> =  vec![];
    let cleaned_file = stripper::strip_comments(file_contents)?;
    let chars_peek_vec: Vec<char> = cleaned_file.chars().collect();
    let mut current_token = BuildToken::None;
    for (index,curr_char) in cleaned_file.chars().enumerate() {
        current_token = match (curr_char, current_token) {
            (' ' | '\t' | '\n', BuildToken::None) => {
                println!("whitespace");
                BuildToken::None
            },
            _ => BuildToken::None
        } 
        
    } 
    Ok(())
    
}

#[cfg(test)]
use rstest::{rstest,fixture};
use std::path::Path;
use std::fs;
#[cfg(test)]
#[fixture]
fn iterativefib_text () -> String {
    let file_path =  Path::new("tests/correct/test1.src");
    fs::read_to_string(file_path).unwrap()
}

#[cfg(test)]
#[rstest]
fn test_scan(iterativefib_text: String) {
    let _ = scan(iterativefib_text);
}