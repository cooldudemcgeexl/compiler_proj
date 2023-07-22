


mod scanner;
mod structs;
use scanner::tokens::TokenType;
use scanner::tokens;

fn main() {
    let test_token = TokenType::PLUS;
    let test_char = '{';
    match test_char {
        'A'..='Z' => println!("YAY!!!!"),
        'a'..='z' => println!("yay!!!!"),
        _ =>  println!(":(")
    }
    println!("{:?}",test_token);
}
