use crate::parser::expression::{Expression, Name};

use super::context::Context;
use super::procedure::AnalyzedProcedureCall;
use super::traits::AnalyzeExpression;
use super::value::Type;
use super::SemanticsError;

#[derive(Debug)]
pub enum AnalyzedExpression {
    BitwiseAnd(Box<AnalyzedExpression>, AnalyzedArithOp),
    BitwiseOr(Box<AnalyzedExpression>, AnalyzedArithOp),
    BitwiseNot(AnalyzedArithOp),
    LogicalAnd(Box<AnalyzedExpression>, AnalyzedArithOp),
    LogicalOr(Box<AnalyzedExpression>, AnalyzedArithOp),
    LogicalNot(AnalyzedArithOp),
    Cast(Box<AnalyzedExpression>, Type),
    BasicExp(AnalyzedArithOp),
}

impl AnalyzedExpression {
    pub fn cast_expr(self, value_type: Type) -> AnalyzedExpression {
        AnalyzedExpression::Cast(Box::new(self), value_type)
    }

    pub fn cond_expr(self, context: &Context) -> Result<AnalyzedExpression, SemanticsError> {
        match self.get_type(context)? {
            Type::Bool => Ok(self),
            Type::Int => Ok(self.cast_expr(Type::Bool)),
            value_type => Err(SemanticsError::InvalidType(
                String::from("Bool,Int"),
                value_type,
            )),
        }
    }
}

impl AnalyzeExpression<Expression> for AnalyzedExpression {
    fn analyze_expression(
        value: Expression,
        context: &mut Context,
    ) -> Result<Self, SemanticsError> {
        match value {
            Expression::BasicExp(expression) => Ok(AnalyzedExpression::BasicExp(AnalyzedArithOp)),
        }
    }
    fn get_type(&self, context: &Context) -> Result<Type, SemanticsError> {
        todo!()
    }
}

#[derive(Debug)]
pub enum AnalyzedArithOp {
    Plus(),
}

#[derive(Debug)]
pub enum AnalyzedRelation {}

#[derive(Debug)]
pub enum AnalyzedTerm {}

#[derive(Debug)]
pub enum AnalyzedFactor {
    NestedExpression(Box<AnalyzedExpression>),
    ProcedureCall(AnalyzedProcedureCall),
}

#[derive(Debug)]
pub enum AnalyzedName {
    Name(String),
    Indexed(String, Box<AnalyzedExpression>),
}

impl AnalyzeExpression<Name> for AnalyzedName {
    fn analyze_expression(value: Name, context: &mut Context) -> Result<Self, SemanticsError> {
        if let Some(expression) = value.expression.as_deref() {
            let expression = AnalyzedExpression::analyze_expression(*expression, context)?;
            let exp_type = expression.get_type(context)?;

            if exp_type != Type::Int {
                Err(SemanticsError::NonIntIndex(
                    value.identifier.identifier_string,
                    exp_type,
                ))
            } else {
                Ok(AnalyzedName::Indexed(
                    value.identifier.identifier_string,
                    Box::new(expression),
                ))
            }
        } else {
            Ok(AnalyzedName::Name(value.identifier.identifier_string))
        }
    }
    fn get_type(&self, context: &Context) -> Result<Type, SemanticsError> {
        match self {
            AnalyzedName::Name(identifier) => {
                context.get_variable_type(identifier).map(Type::clone)
            }
            AnalyzedName::Indexed(identifier, _) => {
                let array_type = context.get_variable_type(identifier)?;
                match array_type {
                    Type::Array(_, _) => Ok(array_type.clone()),
                    _ => Err(SemanticsError::IndexOnNonArray(identifier.to_owned())),
                }
            }
        }
    }
}
