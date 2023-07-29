


mod scanner;
mod parser;
mod tokens;
use std::{path::Path, fs, io};
use thiserror::Error;
use std::collections::VecDeque;


#[derive(Error, Debug)]
enum CompilerError {
    #[error("IO error encountered")]
    FileError(#[from] io::Error),
    #[error("Scanner encountered error")]
    ScannerError(#[from] scanner::ScannerError)
}


fn main() -> Result<(),CompilerError> {
    let file_path =  Path::new("tests/correct/test1.src");
    let file_name = file_path.to_str();
    let mut file_contents = fs::read_to_string(file_path)?;
    let scanner_result = scanner::scan(file_contents)?;
    let token_deque = VecDeque::from(scanner_result);
    
    Ok(())
}
