use std::{fs::File, path::Path};

pub struct InputFile {
    file_contents: String,
    file_name: String,
    line_count: i32,
}

impl InputFile {

    pub fn attach_file(file_name: String) -> bool {
        todo!()
    }

    pub fn get_char() -> char {
        todo!()
    }

    pub fn unget_char(curr_char: char) {
        todo!()
    }

    pub fn inc_line_count() {
        todo!()
    }

    pub fn get_line_count() {
        todo!()
    }
}
