


mod scanner;
mod structs;
use scanner::tokens::TokenType;
use std::{path::{PathBuf, Path}, fs, io};

#[derive(Debug)]
enum CompilerError {
    FileError(io::Error),
    ScannerError(scanner::ScannerError)
}

impl From<io::Error> for CompilerError {
    fn from(value: io::Error) -> Self {
        CompilerError::FileError(value)
    }
}

impl From<scanner::ScannerError> for CompilerError {
    fn from(value: scanner::ScannerError) -> Self {
        CompilerError::ScannerError(value)
    }
}
fn main() -> Result<(),CompilerError> {
    let file_path =  Path::new("/home/nick/repos/compilerT/compiler_proj/tests/correct/test1.src");
    let file_name = file_path.to_str();
    let mut file_contents = fs::read_to_string(file_path)?;
    let scanner_result = scanner::scan(file_contents)?;
    Ok(())
}
