use thiserror::Error;
pub enum StripState {
    Normal,

}

#[derive(Error, Debug)]
pub enum StripError {

}


pub fn strip_comments(file_str: String) -> Result<String, StripError>{
    let mut ret_str = String::with_capacity(file_str.capacity()); // Make a blank string with the same size as the original

    Ok(String::from(""))
}