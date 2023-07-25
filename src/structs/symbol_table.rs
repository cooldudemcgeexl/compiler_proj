use crate::scanner::tokens::TokenType;
use std::collections::HashMap;

pub struct SymbolTable {
    symbol_tab: HashMap<String, TokenType>,
}

impl SymbolTable   {
    pub fn hash_lookup(token_str: String) -> TokenType {
        todo!()
    }
    pub fn enter_scrope() {
        todo!()
    }
    pub fn exit_scope() {
        todo!()
    }
}