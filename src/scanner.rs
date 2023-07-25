use std::{path::Path, };

pub mod scanner_file;
pub mod tokens;
pub mod stripper;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScannerError {
    #[error("Comment stripper encountered error")]
    StripError(#[from] stripper::StripError)
}

pub fn scan(file_contents: String) -> Result<(),ScannerError> {
    let mut line_number = 0u32;
    let cleaned_file = stripper::strip_comments(file_contents)?;
    let chars_peek_vec: Vec<char> = cleaned_file.chars().collect();
    for (index,curr_char) in cleaned_file.chars().enumerate() {
        println!("{} - {}",index,curr_char);
        println!("{} - {}",index,chars_peek_vec[index])
        
    } 
    Ok(())
    
}
