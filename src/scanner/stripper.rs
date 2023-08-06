use thiserror::Error;

pub enum StripState {
    /// Stripper has not encountered a comment character yet.
    /// All characters found in this state will be pushed to the return value.
    Normal,
    /// Stripper has encountered a slash.
    /// Temporary state, as it may not indicate a comment yet.
    FirstSlash,
    /// Stripper is in a line comment.
    LineComment,
    /// Stipper is in a block comment.
    /// Using u8 here to represent comment nesting depth.
    /// Could be larger if we wanted to go insane. u128 anyone?
    BlockComment(u8),
    /// Stripper has encountered a slash within a block comment.
    /// Similar to first slash, but with depth level.
    /// Used to see if we're entering a nesting level.
    BlockCommentSlash(u8),
    /// Stripper has encountered a star in a block comment.
    /// Either means we are increasing block comment level, or exiting a block comment level.
    BlockCommentStar(u8),
}

#[derive(Error, Debug)]
pub enum StripError {
    #[error("Reached max block comment nesting depth. Why?")]
    MaxCommentDepth,
}

pub fn strip_comments(file_str: String) -> Result<String, StripError> {
    let mut ret_str = String::with_capacity(file_str.capacity()); // Make a blank string with the same size as the original
    let mut strip_state = StripState::Normal;

    for char in file_str.chars() {
        strip_state = match (char, strip_state) {
            ('/', StripState::Normal) => StripState::FirstSlash, // This essentially gives a lookahead state. The slash won't be pushed... yet.
            (_, StripState::Normal) => {
                ret_str.push(char);
                StripState::Normal
            }

            ('/', StripState::FirstSlash) => StripState::LineComment, // We found the second slash. We're in a line comment.
            ('*', StripState::FirstSlash) => StripState::BlockComment(1), // Found a star. Block comment.
            (_, StripState::FirstSlash) => {
                // False alarm. Add the slash to the return string, and the current character.
                ret_str.push('/');
                ret_str.push(char);
                StripState::Normal
            }

            ('\n', StripState::LineComment) => {
                // Go back to normal once we find a newline
                ret_str.push(char);
                StripState::Normal
            }
            (_, StripState::LineComment) => StripState::LineComment,

            ('/', StripState::BlockComment(n)) => StripState::BlockCommentSlash(n),
            ('*', StripState::BlockComment(u8::MAX)) => return Err(StripError::MaxCommentDepth),
            ('*', StripState::BlockComment(n)) => StripState::BlockCommentStar(n),
            (_, StripState::BlockComment(n)) => StripState::BlockComment(n),

            ('*', StripState::BlockCommentSlash(n)) => StripState::BlockComment(n + 1),
            (_, StripState::BlockCommentSlash(n)) => StripState::BlockComment(n),

            ('/', StripState::BlockCommentStar(1)) => StripState::Normal,
            ('/', StripState::BlockCommentStar(n)) => StripState::BlockComment(n - 1),
            (_, StripState::BlockCommentStar(n)) => StripState::BlockComment(n),
        }
    }

    Ok(String::from(ret_str))
}

#[cfg(test)]
#[derive(Error, Debug)]
enum StripTestError {
    #[error("Encountered stripping error")]
    StripError(#[from] StripError),
    #[error("Encountered io error")]
    FileError(#[from] std::io::Error),
}
#[cfg(test)]
use rstest::rstest;
use std::path::{Path, PathBuf};
use std::{fs, io};
#[cfg(test)]
#[rstest]

fn strip_comment_test(
    #[files("tests/correct/*.src")] source_file: PathBuf,
) -> Result<(), StripTestError> {
    let file_name = source_file.file_name().unwrap().to_str().unwrap();
    let stripped_name = format!("tests/comment_stripped/{file_name}");
    let stripped_file = Path::new(stripped_name.as_str());

    let source_contents = fs::read_to_string(source_file)?;
    let stripped_contents = fs::read_to_string(stripped_file)?;

    let stripped_source = strip_comments(source_contents)?;
    assert_eq!(stripped_source, stripped_contents);

    Ok(())
}
