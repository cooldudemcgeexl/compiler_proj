use std::fs::File;

pub struct InputFile {
    file: Option<File>,
    file_name: String,
    line_count: i32
}

impl Default for InputFile {
    fn default() -> Self {
        InputFile { file: None, file_name: String::from(""), line_count: 0 }
    }
}