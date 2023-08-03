use super::SemanticsError;
use crate::parser::types::TypeMark;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Bool,
    Int,
    Float,
    String,
    Array(Box<Type>, usize),
    Void,
}

impl Type {
    pub fn expect_type(self, other: Type) -> Result<Self, SemanticsError> {
        if self != other {
            Err(SemanticsError::TypeMismatch(other, self))
        } else {
            Ok(self)
        }
    }

    pub fn can_assign(&self, other: &Type) -> bool {
        if self == other {
            true
        } else {
            match (self, other) {
                (Type::Int, Type::Bool) | (Type::Bool, Type::Int) => true,
                (Type::Float, Type::Int) | (Type::Int, Type::Float) => true,

                _ => false,
            }
        }
    }
}

impl From<TypeMark> for Type {
    fn from(value: TypeMark) -> Self {
        match value {
            TypeMark::Integer => Type::Int,
            TypeMark::Float => Type::Float,
            TypeMark::String => Type::String,
            TypeMark::Bool => Type::Bool,
        }
    }
}

impl From<&TypeMark> for Type {
    fn from(value: &TypeMark) -> Self {
        match value {
            TypeMark::Integer => Type::Int,
            TypeMark::Float => Type::Float,
            TypeMark::String => Type::String,
            TypeMark::Bool => Type::Bool,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NamedValue(pub String, pub Type);

#[derive(Debug, Clone)]
pub struct ProcedureSignature(pub Vec<NamedValue>, pub Type);
