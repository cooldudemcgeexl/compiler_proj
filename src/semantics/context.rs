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

    pub fn set_procedure(
        &mut self,
        is_global: bool,
        identifier: String,
        signature: ProcedureSignature,
    ) -> Result<(), SemanticsError> {
        let procedures = if is_global {
            &mut self.global_scope.procedures
        } else {
            &mut self.local_scope.procedures
        };

        if procedures.contains_key(&identifier) {
            Err(SemanticsError::Redeclared(identifier))
        } else {
            procedures.insert(identifier, signature);
            Ok(())
        }
    }

    pub fn get_variable_type(&self, identifier: &str) -> Result<&Type, SemanticsError> {
        self.local_scope
            .variables
            .get(identifier)
            .or_else(|| self.global_scope.variables.get(identifier))
            .ok_or_else(|| SemanticsError::UndefinedRef(String::from(identifier)))
    }

    pub fn get_procedure_signature(
        &self,
        identifier: &str,
    ) -> Result<&ProcedureSignature, SemanticsError> {
        self.local_scope
            .procedures
            .get(identifier)
            .or_else(|| self.global_scope.procedures.get(identifier))
            .ok_or_else(|| SemanticsError::UndefinedRef(String::from(identifier)))
    }

    pub fn get_return_type(&self) -> &Type {
        &self.local_scope.return_type
    }

    pub fn start_stack(&mut self, return_type: Type) {
        let previous_stack =
            std::mem::replace(&mut self.local_scope, ScopeContext::new(return_type));
        self.scope_stack.push(previous_stack);
    }

    pub fn end_stack(&mut self) -> Result<ScopeContext, SemanticsError> {
        if let Some(scope) = self.scope_stack.pop() {
            let old_scope = std::mem::replace(&mut self.local_scope, scope);
            Ok(old_scope)
        } else {
            Err(SemanticsError::OutOfScope)
        }
    }
}
