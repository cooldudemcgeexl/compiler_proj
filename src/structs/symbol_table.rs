use crate::scanner::tokens::Token;
use std::collections::HashMap;

pub struct SymbolTable {
    symbol_tab: HashMap<String, Token>,
}

impl SymbolTable   {
    pub fn hash_lookup(token_str: String) -> Token {
        todo!()
    }
    pub fn enter_scrope() {
        todo!()
    }
    pub fn exit_scope() {
        todo!()
    }
}