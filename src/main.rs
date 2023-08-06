#![feature(box_patterns)]

mod parser;
mod scanner;
mod semantics;
mod tokens;
use std::collections::VecDeque;
use std::path::Path;
use std::process::exit;
use std::{env, fs, io};
use thiserror::Error;

#[derive(Error, Debug)]
enum CompilerError {
    #[error(transparent)]
    FileError(#[from] io::Error),
    #[error(transparent)]
    ScannerError(#[from] scanner::ScannerError),
    #[error(transparent)]
    ParserError(#[from] parser::utils::ParserError),
    #[error(transparent)]
    SemanticsError(#[from] semantics::SemanticsError),
    #[error(transparent)]
    ArgumentError(#[from] ArgumentError),
}
#[derive(Error, Debug)]
enum ArgumentError {
    #[error("Expected at least one file argument to compile, but found none!")]
    NoArguments,
    #[error(
        "Expected input file and optional output file argument, but found more than 2 arguments!"
    )]
    TooManyArguments,
    #[error("Input file does not exist")]
    FileDoesNotExist,
}
fn main() {
    let main_result = run_comp();
    if let Err(err) = main_result {
        eprintln!("{}", err);
        exit(1)
    }
}

fn run_comp() -> Result<(), CompilerError> {
    let (parsed_input, parsed_output) = parse_args()?;

    let main_result = compile_file(&parsed_input)?;
    Ok(())
}

fn parse_args() -> Result<(PathBuf, PathBuf), ArgumentError> {
    let mut args = env::args().skip(1);
    let input_filename_opt = args.next();
    let output_filename_opt = args.next();

    let input_path = match input_filename_opt {
        None => return Err(ArgumentError::NoArguments),
        Some(input_filename) => PathBuf::from(input_filename),
    };
    if args.len() > 0 {
        return Err(ArgumentError::TooManyArguments);
    }
    if !input_path.is_file() {
        return Err(ArgumentError::FileDoesNotExist);
    }

    let output_path = match output_filename_opt {
        Some(output_filename) => {
            let mut output_path = PathBuf::from(output_filename);
            if output_path.is_dir() {
                // This can be unwrapped, since we checked that `input_path` is a file earlier.
                output_path.push(input_path.file_name().unwrap());
            }
            output_path
        }
        None => {
            let mut output_path = input_path.clone();
            output_path.set_extension("out");
            output_path
        }
    };

    Ok((input_path, output_path))
}

fn compile_file(file_path: &Path) -> Result<(), CompilerError> {
    let file_name = file_path.to_str();
    let mut file_contents = fs::read_to_string(file_path)?;
    let scanner_result = scanner::scan(file_contents)?;
    let token_deque = VecDeque::from(scanner_result);
    let program_struct = parser::parse_tokens(token_deque)?;
    let anayzed_program = semantics::AnalyzedProgram::analyze(program_struct)?;
    println!("{:?}", anayzed_program);

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
