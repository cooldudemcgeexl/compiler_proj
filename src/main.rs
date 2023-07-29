mod parser;
mod scanner;
mod tokens;
use std::collections::VecDeque;
use std::{fs, io, path::Path};
use thiserror::Error;

#[derive(Error, Debug)]
enum CompilerError {
    #[error(transparent)]
    FileError(#[from] io::Error),
    #[error(transparent)]
    ScannerError(#[from] scanner::ScannerError),
    #[error(transparent)]
    ParserError(#[from] parser::utils::ParserError),
}

fn main() -> Result<(), CompilerError> {
    let file_path = Path::new("tests/correct/test1.src");
    let file_name = file_path.to_str();
    let mut file_contents = fs::read_to_string(file_path)?;

    let scanner_result = scanner::scan(file_contents)?;
    let token_deque = VecDeque::from(scanner_result);

    let program_struct = parser::parse_tokens(token_deque)?;

    Ok(())
}
