use std::collections::HashMap;

use thiserror::Error;

use super::{
    value::{NamedValue, ProcedureSignature, Type},
    SemanticsError,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Scope {
    Global,
    Local,
}

#[derive(Debug)]
pub struct ScopeContext {
    pub variables: HashMap<String, Type>,
    pub procedures: HashMap<String, ProcedureSignature>,
    pub return_type: Type,
}

impl ScopeContext {
    pub fn new(return_type: Type) -> Self {
        ScopeContext {
            variables: HashMap::new(),
            procedures: HashMap::new(),
            return_type,
        }
    }

    pub fn new_global_ctx() -> Self {
        let mut procedures = HashMap::new();

        procedures.insert(
            String::from("getbool"),
            ProcedureSignature(vec![], Type::Bool),
        );
        procedures.insert(
            String::from("getinteger"),
            ProcedureSignature(vec![], Type::Int),
        );
        procedures.insert(
            String::from("getfloat"),
            ProcedureSignature(vec![], Type::Float),
        );
        procedures.insert(
            String::from("getstring"),
            ProcedureSignature(vec![], Type::String),
        );

        procedures.insert(
            String::from("putbool"),
            ProcedureSignature(
                vec![NamedValue(String::from("value"), Type::Bool)],
                Type::Bool,
            ),
        );
        procedures.insert(
            String::from("putinteger"),
            ProcedureSignature(
                vec![NamedValue(String::from("value"), Type::Int)],
                Type::Bool,
            ),
        );
        procedures.insert(
            String::from("putfloat"),
            ProcedureSignature(
                vec![NamedValue(String::from("value"), Type::Float)],
                Type::Bool,
            ),
        );
        procedures.insert(
            String::from("putstring"),
            ProcedureSignature(
                vec![NamedValue(String::from("value"), Type::String)],
                Type::Bool,
            ),
        );

        procedures.insert(
            String::from("sqrt"),
            ProcedureSignature(
                vec![NamedValue(String::from("value"), Type::Int)],
                Type::Float,
            ),
        );

        ScopeContext {
            variables: HashMap::new(),
            procedures,
            return_type: Type::Void,
        }
    }
}

#[derive(Debug)]
pub struct Context {
    global_scope: ScopeContext,
    scope_stack: Vec<ScopeContext>,
    local_scope: ScopeContext,
}
impl Context {
    pub fn new() -> Self {
        Context {
            global_scope: ScopeContext::new_global_ctx(),
            scope_stack: Vec::new(),
            local_scope: ScopeContext::new(Type::Void),
        }
    }

    pub fn into_global(self) -> ScopeContext {
        self.global_scope
    }

    pub fn set_type(
        &mut self,
        is_global: bool,
        identifier: String,
        value_type: Type,
    ) -> Result<(), SemanticsError> {
        let variables = if is_global {
            &mut self.global_scope.variables
        } else {
            &mut self.local_scope.variables
        };
        if variables.contains_key(&identifier) {
            Err(SemanticsError::Redeclared(identifier))
        } else {
            variables.insert(identifier, value_type);
            Ok(())
        }
    }
}
