#![feature(box_patterns)]

mod parser;
mod scanner;
mod semantics;
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
    compile_file(file_path)?;

    Ok(())
}

fn compile_file(file_path: &Path) -> Result<(), CompilerError> {
    let file_name = file_path.to_str();
    let mut file_contents = fs::read_to_string(file_path)?;
    let scanner_result = scanner::scan(file_contents)?;
    let token_deque = VecDeque::from(scanner_result);
    let program_struct = parser::parse_tokens(token_deque)?;
    Ok(())
}

#[cfg(test)]
use rstest::rstest;
use std::path::PathBuf;
#[cfg(test)]
#[rstest]

fn compile_test_correct(
    #[files("tests/correct/*.src")] source_file: PathBuf,
) -> Result<(), CompilerError> {
    compile_file(source_file.as_path())
}

#[cfg(test)]
#[rstest]
fn compile_test_incorrect(
    #[files("tests/incorrect/*.src")] source_file: PathBuf,
) -> Result<(), CompilerError> {
    compile_file(source_file.as_path())
}
