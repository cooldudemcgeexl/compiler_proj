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
    let file_chars_vec: Vec<char> = file_contents.chars().collect();
    for (index,curr_char) in file_contents.chars().enumerate() {
        println!("{} - {}",index,curr_char);
        println!("{} - {}",index,file_chars_vec[index])
        
    } 
    Ok(())
    
}
